'use client';

import React, { useState, useEffect } from 'react';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Badge } from '@/components/ui/badge';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useToast } from '@/components/ui/use-toast';
import { Plus, MoreHorizontal, Edit, Trash2, Settings, Eye } from 'lucide-react';
import {
  LifecycleType,
  LifecycleTypeResponse,
  LifecycleTypeSummary,
  CreateLifecycleTypeRequest,
  UpdateLifecycleTypeRequest,
  LifecycleState,
  CreateLifecycleStateRequest,
} from '@/lib/types';

export default function LifecycleManagementPage() {
  const [lifecycles, setLifecycles] = useState<LifecycleTypeSummary[]>([]);
  const [selectedLifecycle, setSelectedLifecycle] = useState<LifecycleTypeResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [isEditDialogOpen, setIsEditDialogOpen] = useState(false);
  const [isStatesDialogOpen, setIsStatesDialogOpen] = useState(false);
  const { toast } = useToast();

  // Form state
  const [formData, setFormData] = useState<CreateLifecycleTypeRequest>({
    name: '',
    description: '',
    default_color: '#3B82F6',
  });

  const [statesForm, setStatesForm] = useState<CreateLifecycleStateRequest>({
    lifecycle_type_id: '',
    name: '',
    description: '',
    color: '#10B981',
    order_index: 0,
    is_initial_state: false,
    is_terminal_state: false,
  });

  const [states, setStates] = useState<LifecycleState[]>([]);
  const [availableColors] = useState([
    '#10B981', // Emerald (green)
    '#3B82F6', // Blue
    '#F59E0B', // Amber (yellow)
    '#EF4444', // Red
    '#8B5CF6', // Violet (purple)
    '#EC4899', // Pink
    '#14B8A6', // Teal
    '#F97316', // Orange
    '#6366F1', // Indigo
    '#84CC16', // Lime
    '#06B6D4', // Cyan
    '#A855F7', // Purple
    '#FB923C', // Orange
    '#0EA5E9', // Sky
    '#22C55E', // Green
  ]);

  // Fetch lifecycles
  const fetchLifecycles = async () => {
    try {
      setIsLoading(true);
      const response = await fetch('/api/v1/lifecycle-types');
      if (!response.ok) throw new Error('Failed to fetch lifecycles');

      const result = await response.json();
      setLifecycles(result.data || []);
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to load lifecycles',
        variant: 'destructive',
      });
    } finally {
      setIsLoading(false);
    }
  };

  // Fetch lifecycle details with states
  const fetchLifecycleDetails = async (id: string) => {
    try {
      const response = await fetch(`/api/v1/lifecycle-types/${id}`);
      if (!response.ok) throw new Error('Failed to fetch lifecycle details');

      const result = await response.json();
      setSelectedLifecycle(result);
      setStates(result.states || []);
      setStatesForm(prev => ({ ...prev, lifecycle_type_id: id }));
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to load lifecycle details',
        variant: 'destructive',
      });
    }
  };

  // Create lifecycle
  const handleCreateLifecycle = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('/api/v1/lifecycle-types', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });

      if (!response.ok) throw new Error('Failed to create lifecycle');

      setIsCreateDialogOpen(false);
      setFormData({ name: '', description: '', default_color: '#3B82F6' });
      fetchLifecycles();

      toast({
        title: 'Success',
        description: 'Lifecycle type created successfully',
      });
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to create lifecycle',
        variant: 'destructive',
      });
    }
  };

  // Update lifecycle
  const handleUpdateLifecycle = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedLifecycle) return;

    try {
      const updateData: UpdateLifecycleTypeRequest = {
        name: formData.name !== selectedLifecycle.name ? formData.name : undefined,
        description: formData.description !== selectedLifecycle.description ? formData.description : undefined,
        default_color: formData.default_color !== selectedLifecycle.default_color ? formData.default_color : undefined,
      };

      const response = await fetch(`/api/v1/lifecycle-types/${selectedLifecycle.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updateData),
      });

      if (!response.ok) throw new Error('Failed to update lifecycle');

      setIsEditDialogOpen(false);
      fetchLifecycles();

      toast({
        title: 'Success',
        description: 'Lifecycle updated successfully',
      });
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to update lifecycle',
        variant: 'destructive',
      });
    }
  };

  // Delete lifecycle
  const handleDeleteLifecycle = async (id: string) => {
    if (!confirm('Are you sure you want to delete this lifecycle? This action cannot be undone.')) {
      return;
    }

    try {
      const response = await fetch(`/api/v1/lifecycle-types/${id}`, {
        method: 'DELETE',
      });

      if (!response.ok) throw new Error('Failed to delete lifecycle');

      fetchLifecycles();

      toast({
        title: 'Success',
        description: 'Lifecycle deleted successfully',
      });
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to delete lifecycle',
        variant: 'destructive',
      });
    }
  };

  // Create lifecycle state
  const handleCreateState = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('/api/v1/lifecycle-states', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(statesForm),
      });

      if (!response.ok) throw new Error('Failed to create lifecycle state');

      setStatesForm({
        ...statesForm,
        name: '',
        description: '',
        color: '#10B981',
        order_index: statesForm.order_index + 1,
        is_initial_state: false,
        is_terminal_state: false,
      });

      fetchLifecycleDetails(statesForm.lifecycle_type_id);

      toast({
        title: 'Success',
        description: 'Lifecycle state created successfully',
      });
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to create lifecycle state',
        variant: 'destructive',
      });
    }
  };

  // Delete lifecycle state
  const handleDeleteState = async (id: string) => {
    if (!confirm('Are you sure you want to delete this state?')) {
      return;
    }

    try {
      const response = await fetch(`/api/v1/lifecycle-states/${id}`, {
        method: 'DELETE',
      });

      if (!response.ok) throw new Error('Failed to delete lifecycle state');

      if (selectedLifecycle) {
        fetchLifecycleDetails(selectedLifecycle.id);
      }

      toast({
        title: 'Success',
        description: 'Lifecycle state deleted successfully',
      });
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to delete lifecycle state',
        variant: 'destructive',
      });
    }
  };

  // Edit lifecycle
  const handleEditLifecycle = (lifecycle: LifecycleTypeSummary) => {
    // Fetch full details for editing
    fetchLifecycleDetails(lifecycle.id);
    setFormData({
      name: lifecycle.name,
      description: lifecycle.description || '',
      default_color: lifecycle.default_color,
    });
    setIsEditDialogOpen(true);
  };

  useEffect(() => {
    fetchLifecycles();
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-semibold text-gray-900">Lifecycle Management</h1>
          <p className="mt-1 text-sm text-gray-600">
            Configure lifecycle types and states for IT asset management
          </p>
        </div>
        <Dialog open={isCreateDialogOpen} onOpenChange={setIsCreateDialogOpen}>
          <DialogTrigger asChild>
            <Button>
              <Plus className="h-4 w-4 mr-2" />
              New Lifecycle Type
            </Button>
          </DialogTrigger>
          <DialogContent className="sm:max-w-[425px]">
            <form onSubmit={handleCreateLifecycle}>
              <DialogHeader>
                <DialogTitle>Create Lifecycle Type</DialogTitle>
                <DialogDescription>
                  Define a new lifecycle type with customizable states and transitions.
                </DialogDescription>
              </DialogHeader>
              <div className="grid gap-4 py-4">
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="name" className="text-right">
                    Name
                  </Label>
                  <Input
                    id="name"
                    value={formData.name}
                    onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                    className="col-span-3"
                    placeholder="e.g., Server Lifecycle"
                    required
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="description" className="text-right">
                    Description
                  </Label>
                  <Textarea
                    id="description"
                    value={formData.description}
                    onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                    className="col-span-3"
                    placeholder="Optional description"
                    rows={3}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="default_color" className="text-right">
                    Default Color
                  </Label>
                  <div className="col-span-3 flex items-center gap-2">
                    <Input
                      id="default_color"
                      type="color"
                      value={formData.default_color}
                      onChange={(e) => setFormData({ ...formData, default_color: e.target.value })}
                      className="h-10 w-20"
                    />
                    <span className="text-sm text-gray-600">{formData.default_color}</span>
                  </div>
                </div>
              </div>
              <DialogFooter>
                <Button type="submit">Create Lifecycle Type</Button>
              </DialogFooter>
            </form>
          </DialogContent>
        </Dialog>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {isLoading ? (
          <div className="col-span-full text-center py-8">
            <div className="text-gray-400">Loading lifecycles...</div>
          </div>
        ) : lifecycles.length === 0 ? (
          <div className="col-span-full text-center py-8">
            <Card>
              <CardContent className="pt-6">
                <div className="text-center">
                  <Settings className="mx-auto h-12 w-12 text-gray-400" />
                  <h3 className="mt-2 text-sm font-medium text-gray-900">No lifecycle types</h3>
                  <p className="mt-1 text-sm text-gray-500">
                    Get started by creating your first lifecycle type.
                  </p>
                </div>
              </CardContent>
            </Card>
          </div>
        ) : (
          lifecycles.map((lifecycle) => (
            <Card key={lifecycle.id} className="relative">
              <CardHeader>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div
                      className="w-4 h-4 rounded-full"
                      style={{ backgroundColor: lifecycle.default_color }}
                    />
                    <CardTitle className="text-lg">{lifecycle.name}</CardTitle>
                  </div>
                  <div className="flex items-center gap-2">
                    <Badge variant={lifecycle.is_active ? "default" : "secondary"}>
                      {lifecycle.is_active ? "Active" : "Inactive"}
                    </Badge>
                    <DropdownMenu>
                      <DropdownMenuTrigger asChild>
                        <Button variant="ghost" className="h-8 w-8 p-0">
                          <MoreHorizontal className="h-4 w-4" />
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent align="end">
                        <DropdownMenuItem onClick={() => fetchLifecycleDetails(lifecycle.id)}>
                          <Eye className="h-4 w-4 mr-2" />
                          View Details
                        </DropdownMenuItem>
                        <DropdownMenuItem onClick={() => handleEditLifecycle(lifecycle)}>
                          <Edit className="h-4 w-4 mr-2" />
                          Edit
                        </DropdownMenuItem>
                        <DropdownMenuItem
                          onClick={() => handleDeleteLifecycle(lifecycle.id)}
                          className="text-red-600"
                        >
                          <Trash2 className="h-4 w-4 mr-2" />
                          Delete
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </div>
                </div>
                {lifecycle.description && (
                  <CardDescription>{lifecycle.description}</CardDescription>
                )}
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">States</span>
                    <span className="font-medium">{lifecycle.state_count}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">CI Types</span>
                    <span className="font-medium">{lifecycle.ci_type_count}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">Created</span>
                    <span className="font-medium">
                      {new Date(lifecycle.created_at).toLocaleDateString()}
                    </span>
                  </div>
                  <div className="pt-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full"
                      onClick={() => {
                        fetchLifecycleDetails(lifecycle.id);
                        setIsStatesDialogOpen(true);
                      }}
                    >
                      <Settings className="h-4 w-4 mr-2" />
                      Manage States
                    </Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))
        )}
      </div>

      {/* Edit Dialog */}
      <Dialog open={isEditDialogOpen} onOpenChange={setIsEditDialogOpen}>
        <DialogContent className="sm:max-w-[425px]">
          <form onSubmit={handleUpdateLifecycle}>
            <DialogHeader>
              <DialogTitle>Edit Lifecycle Type</DialogTitle>
              <DialogDescription>
                Update the lifecycle type configuration.
              </DialogDescription>
            </DialogHeader>
            <div className="grid gap-4 py-4">
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="edit_name" className="text-right">
                  Name
                </Label>
                <Input
                  id="edit_name"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="col-span-3"
                  required
                />
              </div>
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="edit_description" className="text-right">
                  Description
                </Label>
                <Textarea
                  id="edit_description"
                  value={formData.description}
                  onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                  className="col-span-3"
                  rows={3}
                />
              </div>
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="edit_color" className="text-right">
                  Default Color
                </Label>
                <div className="col-span-3 flex items-center gap-2">
                  <Input
                    id="edit_color"
                    type="color"
                    value={formData.default_color}
                    onChange={(e) => setFormData({ ...formData, default_color: e.target.value })}
                    className="h-10 w-20"
                  />
                  <span className="text-sm text-gray-600">{formData.default_color}</span>
                </div>
              </div>
            </div>
            <DialogFooter>
              <Button type="submit">Update Lifecycle Type</Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>

      {/* States Management Dialog */}
      <Dialog open={isStatesDialogOpen} onOpenChange={setIsStatesDialogOpen}>
        <DialogContent className="sm:max-w-[600px]">
          <DialogHeader>
            <DialogTitle>Manage Lifecycle States</DialogTitle>
            <DialogDescription>
              Configure states and transitions for this lifecycle type.
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            {selectedLifecycle && (
              <div className="border rounded-lg p-4">
                <h4 className="font-medium mb-2">{selectedLifecycle.name}</h4>
                <p className="text-sm text-gray-600">{selectedLifecycle.description}</p>
              </div>
            )}

            {/* Add new state form */}
            <div className="border rounded-lg p-4 bg-gray-50">
              <h4 className="font-medium mb-3">Add New State</h4>
              <form onSubmit={handleCreateState} className="space-y-3">
                <div className="grid grid-cols-2 gap-3">
                  <Input
                    placeholder="State name"
                    value={statesForm.name}
                    onChange={(e) => setStatesForm({ ...statesForm, name: e.target.value })}
                    required
                  />
                  <Input
                    type="number"
                    placeholder="Order"
                    value={statesForm.order_index}
                    onChange={(e) => setStatesForm({ ...statesForm, order_index: parseInt(e.target.value) })}
                    min={0}
                    required
                  />
                </div>
                <div className="grid grid-cols-2 gap-3">
                  <select
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background"
                    value={statesForm.color}
                    onChange={(e) => setStatesForm({ ...statesForm, color: e.target.value })}
                  >
                    {availableColors.map((color) => (
                      <option key={color} value={color}>
                        {color}
                      </option>
                    ))}
                  </select>
                  <div className="flex gap-2">
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        checked={statesForm.is_initial_state}
                        onChange={(e) => setStatesForm({ ...statesForm, is_initial_state: e.target.checked })}
                      />
                      Initial
                    </label>
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        checked={statesForm.is_terminal_state}
                        onChange={(e) => setStatesForm({ ...statesForm, is_terminal_state: e.target.checked })}
                      />
                      Terminal
                    </label>
                  </div>
                </div>
                <Textarea
                  placeholder="Description (optional)"
                  value={statesForm.description}
                  onChange={(e) => setStatesForm({ ...statesForm, description: e.target.value })}
                  rows={2}
                />
                <Button type="submit" size="sm" className="w-full">
                  <Plus className="h-4 w-4 mr-2" />
                  Add State
                </Button>
              </form>
            </div>

            {/* States list */}
            <div>
              <h4 className="font-medium mb-3">Current States</h4>
              {states.length === 0 ? (
                <div className="text-center py-4 text-gray-500 text-sm">
                  No states defined yet. Add your first state above.
                </div>
              ) : (
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>State</TableHead>
                      <TableHead>Color</TableHead>
                      <TableHead>Order</TableHead>
                      <TableHead>Type</TableHead>
                      <TableHead className="text-right">Actions</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {states
                      .sort((a, b) => a.order_index - b.order_index)
                      .map((state) => (
                        <TableRow key={state.id}>
                          <TableCell className="font-medium">{state.name}</TableCell>
                          <TableCell>
                            <div className="flex items-center gap-2">
                              <div
                                className="w-4 h-4 rounded-full"
                                style={{ backgroundColor: state.color }}
                              />
                              <span className="text-sm">{state.color}</span>
                            </div>
                          </TableCell>
                          <TableCell>{state.order_index}</TableCell>
                          <TableCell>
                            <div className="flex gap-1">
                              {state.is_initial_state && (
                                <Badge variant="secondary" className="text-xs">Initial</Badge>
                              )}
                              {state.is_terminal_state && (
                                <Badge variant="destructive" className="text-xs">Terminal</Badge>
                              )}
                            </div>
                          </TableCell>
                          <TableCell className="text-right">
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={() => handleDeleteState(state.id)}
                              className="text-red-600 hover:text-red-700"
                            >
                              <Trash2 className="h-4 w-4" />
                            </Button>
                          </TableCell>
                        </TableRow>
                      ))}
                  </TableBody>
                </Table>
              )}
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsStatesDialogOpen(false)}>
              Close
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}