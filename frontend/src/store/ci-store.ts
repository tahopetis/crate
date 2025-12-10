import { create } from 'zustand';
import { CIType, CILifecycle, CIAsset, RelationshipType, Relationship, CIFilters } from '@/lib/types';
import { apiClient } from '@/lib/api';

interface CIStore {
  // State
  ciTypes: CIType[];
  lifecycles: CILifecycle[];
  assets: CIAsset[];
  relationshipTypes: RelationshipType[];
  filters: CIFilters;
  loading: {
    ciTypes: boolean;
    lifecycles: boolean;
    assets: boolean;
    relationshipTypes: boolean;
  };
  errors: {
    ciTypes?: string;
    lifecycles?: string;
    assets?: string;
    relationshipTypes?: string;
  };

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
  filters: {},
  loading: {
    ciTypes: false,
    lifecycles: false,
    assets: false,
    relationshipTypes: false,
  },
  errors: {},

  // CI Types
  fetchCITypes: async () => {
    set((state) => ({ loading: { ...state.loading, ciTypes: true } }));
    try {
      const data = await apiClient.get('/ci-types') as CIType[];
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
    const result = await apiClient.post('/ci-types', data);
    set((state) => ({ ciTypes: [...state.ciTypes, result] }));
    return result;
  },

  updateCIType: async (id, data) => {
    const result = await apiClient.put(`/ci-types/${id}`, data);
    set((state) => ({
      ciTypes: state.ciTypes.map((type) => type.id === id ? result : type)
    }));
    return result;
  },

  deleteCIType: async (id) => {
    await apiClient.delete(`/ci-types/${id}`);
    set((state) => ({
      ciTypes: state.ciTypes.filter((type) => type.id !== id)
    }));
  },

  // Lifecycles
  fetchLifecycles: async () => {
    set((state) => ({ loading: { ...state.loading, lifecycles: true } }));
    try {
      const data = await apiClient.get('/lifecycle-types');
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
    const result = await apiClient.post('/lifecycle-types', data);
    set((state) => ({ lifecycles: [...state.lifecycles, result] }));
    return result;
  },

  updateLifecycle: async (id, data) => {
    const result = await apiClient.put(`/lifecycle-types/${id}`, data);
    set((state) => ({
      lifecycles: state.lifecycles.map((lifecycle) => lifecycle.id === id ? result : lifecycle)
    }));
    return result;
  },

  deleteLifecycle: async (id) => {
    await apiClient.delete(`/lifecycle-types/${id}`);
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
      const endpoint = `/ci-assets${params.toString() ? `?${params.toString()}` : ''}`;
      const data = await apiClient.get(endpoint);
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
    const result = await apiClient.post('/ci-assets', data);
    set((state) => ({ assets: [...state.assets, result] }));
    return result;
  },

  updateAsset: async (id, data) => {
    const result = await apiClient.put(`/ci-assets/${id}`, data);
    set((state) => ({
      assets: state.assets.map((asset) => asset.id === id ? result : asset)
    }));
    return result;
  },

  deleteAsset: async (id) => {
    await apiClient.delete(`/ci-assets/${id}`);
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
      const data = await apiClient.get('/relationship-types');
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
    const result = await apiClient.post('/relationship-types', data);
    set((state) => ({ relationshipTypes: [...state.relationshipTypes, result] }));
    return result;
  },

  updateRelationshipType: async (id, data) => {
    const result = await apiClient.put(`/relationship-types/${id}`, data);
    set((state) => ({
      relationshipTypes: state.relationshipTypes.map((type) => type.id === id ? result : type)
    }));
    return result;
  },

  deleteRelationshipType: async (id) => {
    await apiClient.delete(`/relationship-types/${id}`);
    set((state) => ({
      relationshipTypes: state.relationshipTypes.filter((type) => type.id !== id)
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
      filters: {},
      errors: {},
    });
  },
}));