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
import { Checkbox } from '@/components/ui/checkbox';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Search, Plus, Edit, Trash2, ArrowUpDown } from 'lucide-react';
import { toast } from 'sonner';
import type {
  RelationshipType,
  RelationshipTypeSummary,
  CreateRelationshipTypeRequest,
  UpdateRelationshipTypeRequest,
  CIType
} from '@/lib/types';

export default function RelationshipTypesPage() {
  const [relationshipTypes, setRelationshipTypes] = useState<RelationshipTypeSummary[]>([]);
  const [ciTypes, setCiTypes] = useState<CIType[]>([]);
  const [loading, setLoading] = useState(true);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingType, setEditingType] = useState<RelationshipType | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [bidirectionalFilter, setBidirectionalFilter] = useState<boolean | undefined>(undefined);
  const [isCreating, setIsCreating] = useState(false);
  const [isUpdating, setIsUpdating] = useState(false);
  const [isDeleting, setIsDeleting] = useState<string | null>(null);

  // Form state
  const [formData, setFormData] = useState<CreateRelationshipTypeRequest>({
    name: '',
    description: '',
    from_ci_type_id: '',
    to_ci_type_id: '',
    is_bidirectional: false,
    reverse_name: '',
  });

  // Fetch data
  useEffect(() => {
    fetchRelationshipTypes();
    fetchCITypes();
  }, []);

  const fetchRelationshipTypes = async () => {
    try {
      setLoading(true);
      const params = new URLSearchParams();
      if (searchTerm) params.append('search', searchTerm);
      if (bidirectionalFilter !== undefined) params.append('is_bidirectional', bidirectionalFilter.toString());

      const response = await fetch(`/api/v1/relationship-types?${params}`);
      const data = await response.json();

      if (data.success) {
        setRelationshipTypes(data.data || []);
      } else {
        toast.error('Failed to fetch relationship types');
      }
    } catch (error) {
      console.error('Error fetching relationship types:', error);
      toast.error('Error loading relationship types');
    } finally {
      setLoading(false);
    }
  };

  const fetchCITypes = async () => {
    try {
      const response = await fetch('/api/v1/ci-types?limit=100');
      const data = await response.json();

      if (data.success) {
        setCiTypes(data.data || []);
      }
    } catch (error) {
      console.error('Error fetching CI types:', error);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // Validation
    if (!formData.name.trim()) {
      toast.error('Name is required');
      return;
    }

    if (formData.is_bidirectional && !formData.reverse_name?.trim()) {
      toast.error('Reverse name is required for bidirectional relationships');
      return;
    }

    if (formData.from_ci_type_id === formData.to_ci_type_id) {
      toast.error('Source and target CI types cannot be the same');
      return;
    }

    try {
      if (editingType) {
        // Update existing relationship type
        setIsUpdating(true);
        const updateData: UpdateRelationshipTypeRequest = {
          name: formData.name,
          description: formData.description,
          from_ci_type_id: formData.from_ci_type_id || undefined,
          to_ci_type_id: formData.to_ci_type_id || undefined,
          is_bidirectional: formData.is_bidirectional,
          reverse_name: formData.reverse_name || undefined,
        };

        const response = await fetch(`/api/v1/relationship-types/${editingType.id}`, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(updateData),
        });

        const result = await response.json();

        if (result.success) {
          toast.success('Relationship type updated successfully');
          setIsDialogOpen(false);
          setEditingType(null);
          fetchRelationshipTypes();
          resetForm();
        } else {
          toast.error(result.message || 'Failed to update relationship type');
        }
      } else {
        // Create new relationship type
        setIsCreating(true);
        const response = await fetch('/api/v1/relationship-types', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(formData),
        });

        const result = await response.json();

        if (result.success) {
          toast.success('Relationship type created successfully');
          setIsDialogOpen(false);
          fetchRelationshipTypes();
          resetForm();
        } else {
          toast.error(result.message || 'Failed to create relationship type');
        }
      }
    } catch (error) {
      console.error('Error saving relationship type:', error);
      toast.error('Error saving relationship type');
    } finally {
      setIsCreating(false);
      setIsUpdating(false);
    }
  };

  const handleEdit = (type: RelationshipTypeSummary) => {
    setEditingType({
      id: type.id,
      name: type.name,
      description: '',
      from_ci_type_id: '',
      to_ci_type_id: '',
      is_bidirectional: type.is_bidirectional,
      reverse_name: type.reverse_name,
      attributes_schema: {},
      created_by: '',
      created_at: '',
      updated_at: '',
    });
    setFormData({
      name: type.name,
      description: '',
      from_ci_type_id: '',
      to_ci_type_id: '',
      is_bidirectional: type.is_bidirectional,
      reverse_name: type.reverse_name || '',
    });
    setIsDialogOpen(true);
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this relationship type?')) {
      return;
    }

    try {
      setIsDeleting(id);
      const response = await fetch(`/api/v1/relationship-types/${id}`, {
        method: 'DELETE',
      });

      const result = await response.json();

      if (result.success) {
        toast.success('Relationship type deleted successfully');
        fetchRelationshipTypes();
      } else {
        toast.error(result.message || 'Failed to delete relationship type');
      }
    } catch (error) {
      console.error('Error deleting relationship type:', error);
      toast.error('Error deleting relationship type');
    } finally {
      setIsDeleting(null);
    }
  };

  const resetForm = () => {
    setFormData({
      name: '',
      description: '',
      from_ci_type_id: '',
      to_ci_type_id: '',
      is_bidirectional: false,
      reverse_name: '',
    });
  };

  const openCreateDialog = () => {
    setEditingType(null);
    resetForm();
    setIsDialogOpen(true);
  };

  const filteredTypes = relationshipTypes.filter(type => {
    const matchesSearch = !searchTerm ||
      type.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      type.description?.toLowerCase().includes(searchTerm.toLowerCase());

    const matchesBidirectional = bidirectionalFilter === undefined ||
      type.is_bidirectional === bidirectionalFilter;

    return matchesSearch && matchesBidirectional;
  });

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Relationship Types</h1>
          <p className="text-muted-foreground">
            Define and manage relationship types between CI assets
          </p>
        </div>
        <Button onClick={openCreateDialog}>
          <Plus className="mr-2 h-4 w-4" />
          Add Relationship Type
        </Button>
      </div>

      {/* Filters */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Filters</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex gap-4 items-end">
            <div className="flex-1">
              <Label htmlFor="search">Search</Label>
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <Input
                  id="search"
                  placeholder="Search relationship types..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10"
                />
              </div>
            </div>
            <div className="w-48">
              <Label htmlFor="bidirectional-filter">Direction</Label>
              <Select
                value={bidirectionalFilter?.toString() || ''}
                onValueChange={(value) => setBidirectionalFilter(value === '' ? undefined : value === 'true')}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All directions" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All directions</SelectItem>
                  <SelectItem value="false">Unidirectional</SelectItem>
                  <SelectItem value="true">Bidirectional</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Relationship Types List */}
      <Card>
        <CardHeader>
          <CardTitle>Relationship Types ({filteredTypes.length})</CardTitle>
          <CardDescription>
            Configure how different CI assets can be related to each other
          </CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="text-center py-8">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto"></div>
              <p className="mt-2 text-muted-foreground">Loading relationship types...</p>
            </div>
          ) : filteredTypes.length === 0 ? (
            <div className="text-center py-8">
              <div className="text-muted-foreground mb-4">
                <ArrowUpDown className="mx-auto h-12 w-12" />
              </div>
              <h3 className="text-lg font-medium mb-2">No relationship types found</h3>
              <p className="text-muted-foreground mb-4">
                {searchTerm || bidirectionalFilter !== undefined
                  ? 'Try adjusting your filters'
                  : 'Get started by creating your first relationship type'
                }
              </p>
              <Button onClick={openCreateDialog}>
                <Plus className="mr-2 h-4 w-4" />
                Add Relationship Type
              </Button>
            </div>
          ) : (
            <div className="space-y-4">
              {filteredTypes.map((type) => (
                <div
                  key={type.id}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-muted/50 transition-colors"
                >
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <h3 className="font-semibold">{type.name}</h3>
                      {type.is_bidirectional && (
                        <Badge variant="secondary">Bidirectional</Badge>
                      )}
                    </div>
                    {type.description && (
                      <p className="text-sm text-muted-foreground mb-2">{type.description}</p>
                    )}
                    <div className="flex items-center gap-4 text-sm text-muted-foreground">
                      {type.from_ci_type_name && type.to_ci_type_name && (
                        <span>
                          {type.from_ci_type_name} → {type.to_ci_type_name}
                        </span>
                      )}
                      {type.is_bidirectional && type.reverse_name && (
                        <span>
                          Reverse: {type.reverse_name}
                        </span>
                      )}
                      <span>
                        {type.relationship_count} relationships
                      </span>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleEdit(type)}
                      disabled={isUpdating !== false}
                    >
                      <Edit className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleDelete(type.id)}
                      disabled={isDeleting !== null}
                    >
                      {isDeleting === type.id ? (
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
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>
              {editingType ? 'Edit Relationship Type' : 'Create Relationship Type'}
            </DialogTitle>
            <DialogDescription>
              Define how CI assets can be related to each other
            </DialogDescription>
          </DialogHeader>

          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label htmlFor="name">Name *</Label>
                <Input
                  id="name"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="e.g., Contains"
                  required
                />
              </div>
              <div>
                <Label htmlFor="reverse_name">Reverse Name</Label>
                <Input
                  id="reverse_name"
                  value={formData.reverse_name}
                  onChange={(e) => setFormData({ ...formData, reverse_name: e.target.value })}
                  placeholder="e.g., Contained by"
                  disabled={!formData.is_bidirectional}
                />
              </div>
            </div>

            <div>
              <Label htmlFor="description">Description</Label>
              <Textarea
                id="description"
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                placeholder="Describe this relationship type..."
                rows={3}
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label htmlFor="from_ci_type">Source CI Type</Label>
                <Select
                  value={formData.from_ci_type_id}
                  onValueChange={(value) => setFormData({ ...formData, from_ci_type_id: value })}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select source CI type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="">Any CI Type</SelectItem>
                    {ciTypes.map((ciType) => (
                      <SelectItem key={ciType.id} value={ciType.id}>
                        {ciType.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label htmlFor="to_ci_type">Target CI Type</Label>
                <Select
                  value={formData.to_ci_type_id}
                  onValueChange={(value) => setFormData({ ...formData, to_ci_type_id: value })}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select target CI type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="">Any CI Type</SelectItem>
                    {ciTypes.map((ciType) => (
                      <SelectItem key={ciType.id} value={ciType.id}>
                        {ciType.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <div className="flex items-center space-x-2">
              <Checkbox
                id="is_bidirectional"
                checked={formData.is_bidirectional}
                onCheckedChange={(checked) => setFormData({
                  ...formData,
                  is_bidirectional: checked as boolean,
                  reverse_name: checked ? formData.reverse_name : ''
                })}
              />
              <Label htmlFor="is_bidirectional">
                This relationship is bidirectional (works in both directions)
              </Label>
            </div>

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
                    {editingType ? 'Updating...' : 'Creating...'}
                  </>
                ) : (
                  editingType ? 'Update Relationship Type' : 'Create Relationship Type'
                )}
              </Button>
            </div>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
}