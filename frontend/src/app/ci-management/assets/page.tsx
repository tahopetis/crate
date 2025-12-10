'use client';

import React, { useState, useEffect } from 'react';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Switch } from '@/components/ui/switch';
import { Search, Plus, Edit, Trash2, Filter, Calendar, Building } from 'lucide-react';
import { toast } from 'sonner';
import { apiClient } from '@/lib/api';
import type {
  CIAssetResponse,
  CreateCIAssetRequest,
  UpdateCIAssetRequest,
  CIAssetFilter,
  CIType
} from '@/lib/types';

interface AttributeField {
  key: string;
  type: 'string' | 'number' | 'boolean' | 'select' | 'date' | 'textarea';
  label: string;
  required: boolean;
  options?: string[];
  defaultValue?: any;
}

export default function CIAssetsPage() {
  const [assets, setAssets] = useState<CIAssetResponse[]>([]);
  const [ciTypes, setCiTypes] = useState<CIType[]>([]);
  const [loading, setLoading] = useState(true);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingAsset, setEditingAsset] = useState<CIAssetResponse | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedCIType, setSelectedCIType] = useState<string>('');
  const [isCreating, setIsCreating] = useState(false);
  const [isUpdating, setIsUpdating] = useState(false);
  const [isDeleting, setIsDeleting] = useState<string | null>(null);
  const [advancedFilters, setAdvancedFilters] = useState(false);
  const [showAdvancedSearch, setShowAdvancedSearch] = useState(false);

  // Form state
  const [formData, setFormData] = useState<CreateCIAssetRequest>({
    ci_type_id: '',
    name: '',
    attributes: {},
  });

  // Advanced filters
  const [filters, setFilters] = useState<CIAssetFilter>({
    search: '',
    ci_type_id: '',
    name: '',
    created_after: '',
    created_before: '',
    limit: 50,
    offset: 0,
  });

  // Generated form fields based on CI type schema
  const [formFields, setFormFields] = useState<AttributeField[]>([]);

  // Fetch data
  useEffect(() => {
    fetchAssets();
    fetchCITypes();
  }, [filters]);

  useEffect(() => {
    fetchAssets();
  }, []);

  const fetchAssets = async () => {
    try {
      setLoading(true);
      const params = new URLSearchParams();

      // Apply filters
      if (filters.search) params.append('search', filters.search);
      if (filters.ci_type_id) params.append('ci_type_id', filters.ci_type_id);
      if (filters.name) params.append('name', filters.name);
      if (filters.created_after) params.append('created_after', filters.created_after);
      if (filters.created_before) params.append('created_before', filters.created_before);
      if (filters.limit) params.append('limit', filters.limit.toString());
      if (filters.offset) params.append('offset', filters.offset.toString());

      const data = await apiClient.get(`/ci-assets?${params}`);
      setAssets(data.data || []);
    } catch (error) {
      console.error('Error fetching CI assets:', error);
      toast.error(error instanceof Error ? error.message : 'Error loading CI assets');
    } finally {
      setLoading(false);
    }
  };

  const fetchCITypes = async () => {
    try {
      const data = await apiClient.get('/ci-types?limit=100');
      setCiTypes(data.data || []);
    } catch (error) {
      console.error('Error fetching CI types:', error);
      toast.error(error instanceof Error ? error.message : 'Error loading CI types');
    }
  };

  const generateFormFields = (ciType: CIType): AttributeField[] => {
    const schema = ciType.attributes?.schema;
    const fields: AttributeField[] = [];

    if (!schema || !schema.properties) {
      return fields;
    }

    Object.entries(schema.properties).forEach(([key, prop]: [string, any]) => {
      const field: AttributeField = {
        key,
        type: prop.type || 'string',
        label: prop.title || key.charAt(0).toUpperCase() + key.slice(1).replace(/_/g, ' '),
        required: schema.required?.includes(key) || false,
        defaultValue: prop.default,
        options: prop.enum,
      };
      fields.push(field);
    });

    return fields;
  };

  const handleCITypeChange = (ciTypeId: string) => {
    const ciType = ciTypes.find(ct => ct.id === ciTypeId);
    if (ciType) {
      setFormFields(generateFormFields(ciType));
      // Reset attributes when CI type changes
      const newAttributes: Record<string, any> = {};
      formFields.forEach(field => {
        if (field.defaultValue !== undefined) {
          newAttributes[field.key] = field.defaultValue;
        }
      });
      setFormData({ ...formData, ci_type_id: ciTypeId, attributes: newAttributes });
    }
  };

  const handleAttributeChange = (key: string, value: any) => {
    setFormData({
      ...formData,
      attributes: {
        ...formData.attributes,
        [key]: value,
      },
    });
  };

  const handleSearch = (term: string) => {
    setSearchTerm(term);
    setFilters({ ...filters, search: term, offset: 0 });
  };

  const handleAdvancedFilter = () => {
    setAdvancedFilters(!advancedFilters);
    if (!advancedFilters) {
      // Reset advanced filters when closing
      setFilters({
        ...filters,
        name: '',
        created_after: '',
        created_before: '',
      });
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // Validation
    if (!formData.name.trim()) {
      toast.error('Asset name is required');
      return;
    }

    if (!formData.ci_type_id) {
      toast.error('CI Type is required');
      return;
    }

    try {
      if (editingAsset) {
        // Update existing asset
        setIsUpdating(true);
        const updateData: UpdateCIAssetRequest = {
          name: formData.name,
          attributes: formData.attributes,
        };

        await apiClient.put(`/ci-assets/${editingAsset.id}`, updateData);
        toast.success('CI asset updated successfully');
        setIsDialogOpen(false);
        setEditingAsset(null);
        fetchAssets();
        resetForm();
      } else {
        // Create new asset
        setIsCreating(true);
        await apiClient.post('/ci-assets', formData);
        toast.success('CI asset created successfully');
        setIsDialogOpen(false);
        fetchAssets();
        resetForm();
      }
    } catch (error) {
      console.error('Error saving CI asset:', error);
      toast.error(error instanceof Error ? error.message : 'Error saving CI asset');
    } finally {
      setIsCreating(false);
      setIsUpdating(false);
    }
  };

  const handleEdit = (asset: CIAssetResponse) => {
    setEditingAsset(asset);
    setFormData({
      ci_type_id: asset.ci_type_id,
      name: asset.name,
      attributes: asset.attributes,
    });

    // Generate form fields for the CI type
    const ciType = ciTypes.find(ct => ct.id === asset.ci_type_id);
    if (ciType) {
      setFormFields(generateFormFields(ciType));
    }

    setIsDialogOpen(true);
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this CI asset?')) {
      return;
    }

    try {
      setIsDeleting(id);
      await apiClient.delete(`/ci-assets/${id}`);
      toast.success('CI asset deleted successfully');
      fetchAssets();
    } catch (error) {
      console.error('Error deleting CI asset:', error);
      toast.error(error instanceof Error ? error.message : 'Error deleting CI asset');
    } finally {
      setIsDeleting(null);
    }
  };

  const resetForm = () => {
    setFormData({
      ci_type_id: '',
      name: '',
      attributes: {},
    });
    setFormFields([]);
  };

  const openCreateDialog = () => {
    setEditingAsset(null);
    resetForm();
    setIsDialogOpen(true);
  };

  const filteredAssets = assets.filter(asset => {
    const matchesSearch = !searchTerm ||
      asset.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      asset.attributes && Object.values(asset.attributes).some(val =>
        typeof val === 'string' && val.toLowerCase().includes(searchTerm.toLowerCase())
      );

    const matchesCIType = !selectedCIType || asset.ci_type_id === selectedCIType;

    return matchesSearch && matchesCIType;
  });

  const renderFormField = (field: AttributeField) => {
    const value = formData.attributes?.[field.key] || field.defaultValue || '';

    switch (field.type) {
      case 'string':
        return field.options && field.options.length > 0 ? (
          <Select
            value={value}
            onValueChange={(val) => handleAttributeChange(field.key, val)}
          >
            <SelectTrigger>
              <SelectValue placeholder={`Select ${field.label}`} />
            </SelectTrigger>
            <SelectContent>
              {field.options.map(option => (
                <SelectItem key={option} value={option}>
                  {option}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        ) : (
          <Input
            type={field.key.includes('email') ? 'email' : 'text'}
            value={value}
            onChange={(e) => handleAttributeChange(field.key, e.target.value)}
            placeholder={`Enter ${field.label}`}
            required={field.required}
          />
        );

      case 'number':
        return (
          <Input
            type="number"
            value={value}
            onChange={(e) => handleAttributeChange(field.key, parseFloat(e.target.value) || 0)}
            placeholder={`Enter ${field.label}`}
            required={field.required}
          />
        );

      case 'boolean':
        return (
          <div className="flex items-center space-x-2">
            <Switch
              id={field.key}
              checked={!!value}
              onCheckedChange={(checked) => handleAttributeChange(field.key, checked)}
            />
            <Label htmlFor={field.key}>{field.label}</Label>
          </div>
        );

      case 'date':
        return (
          <Input
            type="date"
            value={value ? new Date(value).toISOString().split('T')[0] : ''}
            onChange={(e) => handleAttributeChange(field.key, e.target.value)}
            required={field.required}
          />
        );

      case 'textarea':
        return (
          <Textarea
            value={value}
            onChange={(e) => handleAttributeChange(field.key, e.target.value)}
            placeholder={`Enter ${field.label}`}
            rows={3}
            required={field.required}
          />
        );

      default:
        return (
          <Input
            value={value}
            onChange={(e) => handleAttributeChange(field.key, e.target.value)}
            placeholder={`Enter ${field.label}`}
            required={field.required}
          />
        );
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">CI Assets</h1>
          <p className="text-muted-foreground">
            Manage and configure your infrastructure assets
          </p>
        </div>
        <Button onClick={openCreateDialog}>
          <Plus className="mr-2 h-4 w-4" />
          Add Asset
        </Button>
      </div>

      {/* Filters */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Filters & Search</CardTitle>
          <CardDescription>
            Search and filter your CI assets
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {/* Basic Search */}
            <div className="flex gap-4 items-end">
              <div className="flex-1">
                <div className="relative">
                  <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                  <Input
                    placeholder="Search assets..."
                    value={searchTerm}
                    onChange={(e) => handleSearch(e.target.value)}
                    className="pl-10"
                  />
                </div>
              </div>
              <div className="w-64">
                <Select
                  value={selectedCIType}
                  onValueChange={setSelectedCIType}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="All CI Types" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="">All CI Types</SelectItem>
                    {ciTypes.map((ciType) => (
                      <SelectItem key={ciType.id} value={ciType.id}>
                        <div className="flex items-center gap-2">
                          {ciType.icon && <span>{ciType.icon}</span>}
                          {ciType.name}
                        </div>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <Button
                variant="outline"
                onClick={handleAdvancedFilter}
              >
                <Filter className="mr-2 h-4 w-4" />
                {advancedFilters ? 'Hide Advanced' : 'Advanced Filters'}
              </Button>
            </div>

            {/* Advanced Filters */}
            {advancedFilters && (
              <div className="grid grid-cols-4 gap-4 pt-4 border-t">
                <div>
                  <Label htmlFor="name-filter">Asset Name</Label>
                  <Input
                    id="name-filter"
                    placeholder="Filter by name"
                    value={filters.name}
                    onChange={(e) => setFilters({ ...filters, name: e.target.value })}
                  />
                </div>
                <div>
                  <Label htmlFor="created-after">Created After</Label>
                  <Input
                    id="created-after"
                    type="date"
                    value={filters.created_after}
                    onChange={(e) => setFilters({ ...filters, created_after: e.target.value })}
                  />
                </div>
                <div>
                  <Label htmlFor="created-before">Created Before</Label>
                  <Input
                    id="created-before"
                    type="date"
                    value={filters.created_before}
                    onChange={(e) => setFilters({ ...filters, created_before: e.target.value })}
                  />
                </div>
                <div className="flex items-end">
                  <Button
                    variant="outline"
                    onClick={() => {
                      setFilters({
                        search: searchTerm,
                        ci_type_id: selectedCIType,
                        name: '',
                        created_after: '',
                        created_before: '',
                        limit: 50,
                        offset: 0,
                      });
                    }}
                  >
                    Clear Filters
                  </Button>
                </div>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Assets List */}
      <Card>
        <CardHeader>
          <CardTitle>Assets ({filteredAssets.length})</CardTitle>
          <CardDescription>
            Your configuration items and infrastructure assets
          </CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="text-center py-8">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto"></div>
              <p className="mt-2 text-muted-foreground">Loading assets...</p>
            </div>
          ) : filteredAssets.length === 0 ? (
            <div className="text-center py-8">
              <div className="text-muted-foreground mb-4">
                <Building className="mx-auto h-12 w-12" />
              </div>
              <h3 className="text-lg font-medium mb-2">No assets found</h3>
              <p className="text-muted-foreground mb-4">
                {searchTerm || selectedCIType
                  ? 'Try adjusting your filters'
                  : 'Get started by creating your first CI asset'
                }
              </p>
              <Button onClick={openCreateDialog}>
                <Plus className="mr-2 h-4 w-4" />
                Add Asset
              </Button>
            </div>
          ) : (
            <div className="space-y-4">
              {filteredAssets.map((asset) => (
                <div
                  key={asset.id}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-muted/50 transition-colors"
                >
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <h3 className="font-semibold">{asset.name}</h3>
                      <Badge variant="outline">{asset.ci_type_name}</Badge>
                    </div>
                    <div className="text-sm text-muted-foreground">
                      <div className="flex items-center gap-4 mb-1">
                        <span>ID: {asset.id}</span>
                        <span>•</span>
                        <span>Created: {new Date(asset.created_at).toLocaleDateString()}</span>
                      </div>
                      {Object.keys(asset.attributes).length > 0 && (
                        <div className="mt-2">
                          <span className="font-medium">Attributes: </span>
                          <span className="text-muted-foreground">
                            {Object.entries(asset.attributes)
                              .slice(0, 3)
                              .map(([key, value]) => `${key}: ${typeof value === 'object' ? JSON.stringify(value) : value}`)
                              .join(', ')}
                            {Object.keys(asset.attributes).length > 3 && '...'}
                          </span>
                        </div>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleEdit(asset)}
                      disabled={isUpdating !== false}
                    >
                      <Edit className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleDelete(asset.id)}
                      disabled={isDeleting !== null}
                    >
                      {isDeleting === asset.id ? (
                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-current"></div>
                      ) : (
                        <Trash2 className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Create/Edit Dialog */}
      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
        <DialogContent className="max-w-2xl max-h-[80vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>
              {editingAsset ? 'Edit CI Asset' : 'Create CI Asset'}
            </DialogTitle>
            <DialogDescription>
              Configure your infrastructure asset with dynamic attributes
            </DialogDescription>
          </DialogHeader>

          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label htmlFor="ci_type_id">CI Type *</Label>
                <Select
                  value={formData.ci_type_id}
                  onValueChange={handleCITypeChange}
                  disabled={!!editingAsset}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select CI type" />
                  </SelectTrigger>
                  <SelectContent>
                    {ciTypes.map((ciType) => (
                      <SelectItem key={ciType.id} value={ciType.id}>
                        <div className="flex items-center gap-2">
                          {ciType.icon && <span>{ciType.icon}</span>}
                          {ciType.name}
                        </div>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label htmlFor="name">Asset Name *</Label>
                <Input
                  id="name"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="e.g., Production Server"
                  required
                />
              </div>
            </div>

            {/* Dynamic form fields based on CI type schema */}
            {formFields.length > 0 && (
              <div className="space-y-4">
                <h4 className="font-medium">Attributes</h4>
                <div className="grid grid-cols-2 gap-4">
                  {formFields.map((field) => (
                    <div key={field.key}>
                      <Label htmlFor={field.key}>
                        {field.label} {field.required && '*'}
                      </Label>
                      {renderFormField(field)}
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Custom attributes fallback */}
            {formFields.length === 0 && formData.ci_type_id && (
              <div className="space-y-4">
                <h4 className="font-medium">Custom Attributes</h4>
                <p className="text-sm text-muted-foreground">
                  This CI type doesn&apos;t have a defined schema. You can add custom attributes.
                </p>
                <div>
                  <Label htmlFor="custom-attributes">Custom Attributes (JSON)</Label>
                  <Textarea
                    id="custom-attributes"
                    value={JSON.stringify(formData.attributes, null, 2)}
                    onChange={(e) => {
                      try {
                        const parsed = JSON.parse(e.target.value);
                        setFormData({ ...formData, attributes: parsed });
                      } catch (error) {
                        // Invalid JSON, ignore
                      }
                    }}
                    placeholder='{"key": "value"}'
                    rows={4}
                  />
                </div>
              </div>
            )}

            <div className="flex justify-end gap-2 pt-4">
              <Button
                type="button"
                variant="outline"
                onClick={() => setIsDialogOpen(false)}
                disabled={isCreating || isUpdating}
              >
                Cancel
              </Button>
              <Button
                type="submit"
                disabled={isCreating || isUpdating}
              >
                {isCreating || isUpdating ? (
                  <>
                    <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                    {editingAsset ? 'Updating...' : 'Creating...'}
                  </>
                ) : (
                  editingAsset ? 'Update Asset' : 'Create Asset'
                )}
              </Button>
            </div>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
}