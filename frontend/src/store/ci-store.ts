import { create } from 'zustand';
import { CIType, CILifecycle, CIAsset, RelationshipType, Relationship, CIFilters } from '@/lib/types';
import { apiClient } from '@/lib/api';

interface CIStore {
  // State
  ciTypes: CIType[];
  lifecycles: CILifecycle[];
  assets: CIAsset[];
  relationshipTypes: RelationshipType[];
  relationships: Relationship[];
  filters: CIFilters;
  loading: {
    ciTypes: boolean;
    lifecycles: boolean;
    assets: boolean;
    relationshipTypes: boolean;
    relationships: boolean;
  };
  errors: {
    ciTypes?: string;
    lifecycles?: string;
    assets?: string;
    relationshipTypes?: string;
    relationships?: string;
  };

  // API helper
  apiRequest: (endpoint: string, options?: RequestInit) => Promise<any>;

  // CI Types
  fetchCITypes: () => Promise<void>;
  createCIType: (data: Partial<CIType>) => Promise<CIType>;
  updateCIType: (id: string, data: Partial<CIType>) => Promise<CIType>;
  deleteCIType: (id: string) => Promise<void>;

  // Lifecycles
  fetchLifecycles: () => Promise<void>;
  createLifecycle: (data: Partial<CILifecycle>) => Promise<CILifecycle>;
  updateLifecycle: (id: string, data: Partial<CILifecycle>) => Promise<CILifecycle>;
  deleteLifecycle: (id: string) => Promise<void>;

  // Assets
  fetchAssets: (filters?: CIFilters) => Promise<void>;
  createAsset: (data: Partial<CIAsset>) => Promise<CIAsset>;
  updateAsset: (id: string, data: Partial<CIAsset>) => Promise<CIAsset>;
  deleteAsset: (id: string) => Promise<void>;
  getAssetById: (id: string) => CIAsset | undefined;

  // Relationship Types
  fetchRelationshipTypes: () => Promise<void>;
  createRelationshipType: (data: Partial<RelationshipType>) => Promise<RelationshipType>;
  updateRelationshipType: (id: string, data: Partial<RelationshipType>) => Promise<RelationshipType>;
  deleteRelationshipType: (id: string) => Promise<void>;

  // Relationships
  fetchRelationships: (ciAssetId?: string) => Promise<void>;
  createRelationship: (data: Partial<Relationship>) => Promise<Relationship>;
  updateRelationship: (id: string, data: Partial<Relationship>) => Promise<Relationship>;
  deleteRelationship: (id: string) => Promise<void>;

  // Filters
  setFilters: (filters: Partial<CIFilters>) => void;
  clearFilters: () => void;

  // Utility
  reset: () => void;
}

export const useCIStore = create<CIStore>((set, get) => ({
  // Initial state
  ciTypes: [],
  lifecycles: [],
  assets: [],
  relationshipTypes: [],
  relationships: [],
  filters: {},
  loading: {
    ciTypes: false,
    lifecycles: false,
    assets: false,
    relationshipTypes: false,
    relationships: false,
  },
  errors: {},

  // API helper function
  apiRequest: async (endpoint: string, options?: RequestInit) => {
    const token = localStorage.getItem('auth-storage')
      ? JSON.parse(localStorage.getItem('auth-storage')!).state.token
      : null;

    const response = await fetch(`/api${endpoint}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` }),
        ...options?.headers,
      },
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({}));
      throw new Error(error.message || 'Request failed');
    }

    return response.json();
  },

  // CI Types
  fetchCITypes: async () => {
    set((state) => ({ loading: { ...state.loading, ciTypes: true } }));
    try {
      const data = await apiClient.get('/ci/types') as CIType[];
      set({ ciTypes: data });
    } catch (error) {
      set((state) => ({
        errors: { ...state.errors, ciTypes: error instanceof Error ? error.message : 'Failed to fetch CI types' }
      }));
    } finally {
      set((state) => ({ loading: { ...state.loading, ciTypes: false } }));
    }
  },

  createCIType: async (data) => {
    const result = await get().apiRequest('/ci/types', {
      method: 'POST',
      body: JSON.stringify(data),
    });
    set((state) => ({ ciTypes: [...state.ciTypes, result] }));
    return result;
  },

  updateCIType: async (id, data) => {
    const result = await get().apiRequest(`/ci/types/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
    set((state) => ({
      ciTypes: state.ciTypes.map((type) => type.id === id ? result : type)
    }));
    return result;
  },

  deleteCIType: async (id) => {
    await get().apiRequest(`/ci/types/${id}`, { method: 'DELETE' });
    set((state) => ({
      ciTypes: state.ciTypes.filter((type) => type.id !== id)
    }));
  },

  // Lifecycles
  fetchLifecycles: async () => {
    set((state) => ({ loading: { ...state.loading, lifecycles: true } }));
    try {
      const data = await get().apiRequest('/ci/lifecycles');
      set({ lifecycles: data });
    } catch (error) {
      set((state) => ({
        errors: { ...state.errors, lifecycles: error instanceof Error ? error.message : 'Failed to fetch lifecycles' }
      }));
    } finally {
      set((state) => ({ loading: { ...state.loading, lifecycles: false } }));
    }
  },

  createLifecycle: async (data) => {
    const result = await get().apiRequest('/ci/lifecycles', {
      method: 'POST',
      body: JSON.stringify(data),
    });
    set((state) => ({ lifecycles: [...state.lifecycles, result] }));
    return result;
  },

  updateLifecycle: async (id, data) => {
    const result = await get().apiRequest(`/ci/lifecycles/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
    set((state) => ({
      lifecycles: state.lifecycles.map((lifecycle) => lifecycle.id === id ? result : lifecycle)
    }));
    return result;
  },

  deleteLifecycle: async (id) => {
    await get().apiRequest(`/ci/lifecycles/${id}`, { method: 'DELETE' });
    set((state) => ({
      lifecycles: state.lifecycles.filter((lifecycle) => lifecycle.id !== id)
    }));
  },

  // Assets
  fetchAssets: async (filters) => {
    set((state) => ({ loading: { ...state.loading, assets: true } }));
    try {
      const params = new URLSearchParams();
      if (filters) {
        Object.entries(filters).forEach(([key, value]) => {
          if (value !== undefined && value !== null) {
            params.append(key, String(value));
          }
        });
      }
      const endpoint = `/ci/assets${params.toString() ? `?${params.toString()}` : ''}`;
      const data = await get().apiRequest(endpoint);
      set({ assets: data.data || data }); // Handle both paginated and non-paginated responses
    } catch (error) {
      set((state) => ({
        errors: { ...state.errors, assets: error instanceof Error ? error.message : 'Failed to fetch assets' }
      }));
    } finally {
      set((state) => ({ loading: { ...state.loading, assets: false } }));
    }
  },

  createAsset: async (data) => {
    const result = await get().apiRequest('/ci/assets', {
      method: 'POST',
      body: JSON.stringify(data),
    });
    set((state) => ({ assets: [...state.assets, result] }));
    return result;
  },

  updateAsset: async (id, data) => {
    const result = await get().apiRequest(`/ci/assets/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
    set((state) => ({
      assets: state.assets.map((asset) => asset.id === id ? result : asset)
    }));
    return result;
  },

  deleteAsset: async (id) => {
    await get().apiRequest(`/ci/assets/${id}`, { method: 'DELETE' });
    set((state) => ({
      assets: state.assets.filter((asset) => asset.id !== id)
    }));
  },

  getAssetById: (id) => {
    return get().assets.find((asset) => asset.id === id);
  },

  // Relationship Types
  fetchRelationshipTypes: async () => {
    set((state) => ({ loading: { ...state.loading, relationshipTypes: true } }));
    try {
      const data = await get().apiRequest('/ci/relationship-types');
      set({ relationshipTypes: data });
    } catch (error) {
      set((state) => ({
        errors: { ...state.errors, relationshipTypes: error instanceof Error ? error.message : 'Failed to fetch relationship types' }
      }));
    } finally {
      set((state) => ({ loading: { ...state.loading, relationshipTypes: false } }));
    }
  },

  createRelationshipType: async (data) => {
    const result = await get().apiRequest('/ci/relationship-types', {
      method: 'POST',
      body: JSON.stringify(data),
    });
    set((state) => ({ relationshipTypes: [...state.relationshipTypes, result] }));
    return result;
  },

  updateRelationshipType: async (id, data) => {
    const result = await get().apiRequest(`/ci/relationship-types/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
    set((state) => ({
      relationshipTypes: state.relationshipTypes.map((type) => type.id === id ? result : type)
    }));
    return result;
  },

  deleteRelationshipType: async (id) => {
    await get().apiRequest(`/ci/relationship-types/${id}`, { method: 'DELETE' });
    set((state) => ({
      relationshipTypes: state.relationshipTypes.filter((type) => type.id !== id)
    }));
  },

  // Relationships
  fetchRelationships: async (ciAssetId) => {
    set((state) => ({ loading: { ...state.loading, relationships: true } }));
    try {
      const endpoint = ciAssetId ? `/ci/relationships?ci_asset_id=${ciAssetId}` : '/ci/relationships';
      const data = await get().apiRequest(endpoint);
      set({ relationships: data });
    } catch (error) {
      set((state) => ({
        errors: { ...state.errors, relationships: error instanceof Error ? error.message : 'Failed to fetch relationships' }
      }));
    } finally {
      set((state) => ({ loading: { ...state.loading, relationships: false } }));
    }
  },

  createRelationship: async (data) => {
    const result = await get().apiRequest('/ci/relationships', {
      method: 'POST',
      body: JSON.stringify(data),
    });
    set((state) => ({ relationships: [...state.relationships, result] }));
    return result;
  },

  updateRelationship: async (id, data) => {
    const result = await get().apiRequest(`/ci/relationships/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
    set((state) => ({
      relationships: state.relationships.map((rel) => rel.id === id ? result : rel)
    }));
    return result;
  },

  deleteRelationship: async (id) => {
    await get().apiRequest(`/ci/relationships/${id}`, { method: 'DELETE' });
    set((state) => ({
      relationships: state.relationships.filter((rel) => rel.id !== id)
    }));
  },

  // Filters
  setFilters: (filters) => {
    set((state) => ({ filters: { ...state.filters, ...filters } }));
  },

  clearFilters: () => {
    set({ filters: {} });
  },

  // Reset
  reset: () => {
    set({
      ciTypes: [],
      lifecycles: [],
      assets: [],
      relationshipTypes: [],
      relationships: [],
      filters: {},
      errors: {},
    });
  },
}));