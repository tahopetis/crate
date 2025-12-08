// User & Authentication
export interface User {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'user';
  createdAt: string;
  updatedAt: string;
}

export interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterData {
  email: string;
  password: string;
  name: string;
}

// CI Management
export interface CIType {
  id: string;
  name: string;
  description?: string;
  icon?: string;
  color?: string;
  attributes: Record<string, any>;
  createdAt: string;
  updatedAt: string;
}

export interface CILifecycle {
  id: string;
  name: string;
  description?: string;
  statuses: LifecycleStatus[];
  createdAt: string;
  updatedAt: string;
}

export interface LifecycleStatus {
  id: string;
  name: string;
  order: number;
  color?: string;
  description?: string;
}

export interface CIAsset {
  id: string;
  name: string;
  ci_type_id: string;
  ci_type?: CIType;
  lifecycle_status_id: string;
  lifecycle_status?: LifecycleStatus;
  attributes: Record<string, any>;
  value?: number;
  purchase_date?: string;
  depreciation_period?: number; // in months
  created_at: string;
  updated_at: string;
}

export interface RelationshipType {
  id: string;
  name: string;
  description?: string;
  is_bidirectional: boolean;
  source_cardinality: 'one' | 'many';
  target_cardinality: 'one' | 'many';
  created_at: string;
  updated_at: string;
}

export interface Relationship {
  id: string;
  relationship_type_id: string;
  relationship_type?: RelationshipType;
  source_ci_id: string;
  source_ci?: CIAsset;
  target_ci_id: string;
  target_ci?: CIAsset;
  attributes: Record<string, any>;
  created_at: string;
  updated_at: string;
}

// Graph Visualization
export interface GraphNode {
  id: string;
  label: string;
  type: string;
  color?: string;
  size?: number;
  data: CIAsset;
}

export interface GraphEdge {
  id: string;
  source: string;
  target: string;
  label: string;
  type: string;
  color?: string;
  data: Relationship;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export interface GraphLayout {
  name: string;
  animate?: boolean;
  fit?: boolean;
  padding?: number;
}

// Audit Log
export interface AuditLog {
  id: string;
  action: 'create' | 'update' | 'delete';
  entity_type: 'ci_type' | 'ci_asset' | 'relationship' | 'lifecycle';
  entity_id: string;
  user_id: string;
  user?: User;
  changes: Record<string, { old: any; new: any }>;
  timestamp: string;
  description?: string;
}

// Amortization & Valuation
export interface ValuationHistory {
  id: string;
  ci_asset_id: string;
  ci_asset?: CIAsset;
  value: number;
  accumulated_depreciation: number;
  net_book_value: number;
  calculation_date: string;
  calculation_method: string;
}

export interface AmortizationCalculation {
  ci_asset_id: string;
  original_value: number;
  purchase_date: string;
  depreciation_period: number;
  method: 'straight_line' | 'declining_balance';
  current_value: number;
  accumulated_depreciation: number;
  monthly_depreciation: number;
}

// API Responses
export interface ApiResponse<T = any> {
  data: T;
  message?: string;
  success: boolean;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  totalPages: number;
}

// UI State
export interface UIState {
  sidebarOpen: boolean;
  theme: 'light' | 'dark' | 'system';
  loading: Record<string, boolean>;
  errors: Record<string, string>;
  notifications: Notification[];
}

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  timestamp: string;
  read: boolean;
}

// Form Types
export interface CITypeFormData {
  name: string;
  description?: string;
  icon?: string;
  color?: string;
  attributes: Record<string, any>;
}

export interface CIAssetFormData {
  name: string;
  ci_type_id: string;
  lifecycle_status_id: string;
  attributes: Record<string, any>;
  value?: number;
  purchase_date?: string;
  depreciation_period?: number;
}

export interface RelationshipFormData {
  relationship_type_id: string;
  source_ci_id: string;
  target_ci_id: string;
  attributes: Record<string, any>;
}

// Filter and Search Types
export interface CIFilters {
  ci_type_id?: string;
  lifecycle_status_id?: string;
  search?: string;
  value_min?: number;
  value_max?: number;
  purchase_date_from?: string;
  purchase_date_to?: string;
}

export interface AuditFilters {
  action?: string;
  entity_type?: string;
  user_id?: string;
  date_from?: string;
  date_to?: string;
  search?: string;
}

// Chart Data Types
export interface ChartData {
  name: string;
  value: number;
  color?: string;
}

export interface TimeSeriesData {
  date: string;
  value: number;
  label?: string;
}