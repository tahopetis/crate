'use client';

import React, { useState, useEffect } from 'react';
import { useCIStore } from '@/store/ci-store';
import { CIType } from '@/lib/types';
import { CITypeFormData } from '@/lib/validations';
import { CITypeForm } from '@/components/ci/ci-type-form';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Alert, AlertDescription } from '@/components/ui/alert';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useToast } from '@/components/ui/use-toast';
import {
  Loader2,
  Plus,
  Search,
  MoreHorizontal,
  Edit,
  Trash2,
  Monitor,
  Database,
  Server,
  Globe,
  Laptop,
  Smartphone,
  Package,
  Settings,
} from 'lucide-react';

const DEFAULT_ICON = Settings;

const getIconComponent = (iconName?: string) => {
  const iconMap: Record<string, React.ElementType> = {
    server: Server,
    database: Database,
    monitor: Monitor,
    globe: Globe,
    laptop: Laptop,
    smartphone: Smartphone,
    package: Package,
  };
  return iconMap[iconName || ''] || DEFAULT_ICON;
};

export default function CITypesPage() {
  const { ciTypes, loading, errors, fetchCITypes, createCIType, updateCIType, deleteCIType } = useCIStore();
  const [searchTerm, setSearchTerm] = useState('');
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingCIType, setEditingCIType] = useState<CIType | null>(null);
  const [deletingCIType, setDeletingCIType] = useState<CIType | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [filteredCITypes, setFilteredCITypes] = useState<CIType[]>([]);
  const { toast } = useToast();

  useEffect(() => {
    fetchCITypes();
  }, [fetchCITypes]);

  useEffect(() => {
    const filtered = ciTypes.filter((ciType) =>
      ciType.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      ciType.description?.toLowerCase().includes(searchTerm.toLowerCase())
    );
    setFilteredCITypes(filtered);
  }, [ciTypes, searchTerm]);

  const handleCreateCIType = () => {
    setEditingCIType(null);
    setIsFormOpen(true);
  };

  const handleEditCIType = (ciType: CIType) => {
    setEditingCIType(ciType);
    setIsFormOpen(true);
  };

  const handleDeleteCIType = (ciType: CIType) => {
    setDeletingCIType(ciType);
  };

  const confirmDelete = async () => {
    if (deletingCIType) {
      setIsSubmitting(true);
      try {
        await deleteCIType(deletingCIType.id);
        toast({
          title: "Success",
          description: `CI Type "${deletingCIType.name}" has been deleted.`,
        });
        setDeletingCIType(null);
      } catch (error) {
        toast({
          variant: "destructive",
          title: "Error",
          description: error instanceof Error ? error.message : "Failed to delete CI Type",
        });
      } finally {
        setIsSubmitting(false);
      }
    }
  };

  const handleFormSubmit = async (data: CITypeFormData) => {
    setIsSubmitting(true);
    try {
      if (editingCIType) {
        await updateCIType(editingCIType.id, data);
        toast({
          title: "Success",
          description: `CI Type "${data.name}" has been updated.`,
        });
      } else {
        await createCIType(data);
        toast({
          title: "Success",
          description: `CI Type "${data.name}" has been created.`,
        });
      }
      await fetchCITypes(); // Refresh the list
    } catch (error) {
      toast({
        variant: "destructive",
        title: "Error",
        description: error instanceof Error ? error.message : "Failed to save CI Type",
      });
      throw error; // Re-throw to let the form handle it
    } finally {
      setIsSubmitting(false);
    }
  };

  const getAttributeCount = (attributes: Record<string, any>) => {
    return Object.keys(attributes).length;
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString();
  };

  if (errors.ciTypes) {
    return (
      <Alert variant="destructive">
        <AlertDescription>{errors.ciTypes}</AlertDescription>
      </Alert>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-semibold text-gray-900">CI Types</h1>
          <p className="mt-1 text-sm text-gray-600">
            Manage your Configuration Item types and their attributes
          </p>
        </div>
        <Button onClick={handleCreateCIType} disabled={loading.ciTypes}>
          <Plus className="w-4 h-4 mr-2" />
          New CI Type
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>CI Types ({filteredCITypes.length})</CardTitle>
          <CardDescription>
            Define the structure and attributes for different types of configuration items
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex items-center space-x-2 mb-6">
            <div className="relative flex-1 max-w-sm">
              <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input
                placeholder="Search CI Types..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-8"
              />
            </div>
          </div>

          {loading.ciTypes ? (
            <div className="flex justify-center items-center py-8">
              <Loader2 className="h-8 w-8 animate-spin" />
              <span className="ml-2">Loading CI Types...</span>
            </div>
          ) : filteredCITypes.length === 0 ? (
            <div className="text-center py-12">
              <Settings className="mx-auto h-12 w-12 text-gray-400 mb-4" />
              <h3 className="text-lg font-medium text-gray-900 mb-2">
                {searchTerm ? 'No CI Types found' : 'No CI Types yet'}
              </h3>
              <p className="text-gray-600 max-w-md mx-auto mb-6">
                {searchTerm
                  ? 'Try adjusting your search terms.'
                  : 'Get started by creating your first CI Type to define the structure of your configuration items.'}
              </p>
              {!searchTerm && (
                <Button onClick={handleCreateCIType}>
                  <Plus className="w-4 h-4 mr-2" />
                  Create CI Type
                </Button>
              )}
            </div>
          ) : (
            <div className="rounded-md border">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead className="w-12"></TableHead>
                    <TableHead>Name</TableHead>
                    <TableHead>Description</TableHead>
                    <TableHead>Attributes</TableHead>
                    <TableHead>Created</TableHead>
                    <TableHead className="w-12"></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {filteredCITypes.map((ciType) => {
                    const IconComponent = getIconComponent(ciType.icon);
                    return (
                      <TableRow key={ciType.id}>
                        <TableCell>
                          <div
                            className="w-8 h-8 rounded-lg flex items-center justify-center"
                            style={{ backgroundColor: `${ciType.color}20` }}
                          >
                            <IconComponent
                              className="w-4 h-4"
                              style={{ color: ciType.color }}
                            />
                          </div>
                        </TableCell>
                        <TableCell className="font-medium">{ciType.name}</TableCell>
                        <TableCell>
                          <div className="max-w-xs truncate">
                            {ciType.description || (
                              <span className="text-muted-foreground italic">No description</span>
                            )}
                          </div>
                        </TableCell>
                        <TableCell>
                          <Badge variant="secondary">
                            {getAttributeCount(ciType.attributes)} attributes
                          </Badge>
                        </TableCell>
                        <TableCell className="text-muted-foreground">
                          {formatDate(ciType.createdAt)}
                        </TableCell>
                        <TableCell>
                          <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                              <Button variant="ghost" className="h-8 w-8 p-0">
                                <MoreHorizontal className="h-4 w-4" />
                              </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent align="end">
                              <DropdownMenuLabel>Actions</DropdownMenuLabel>
                              <DropdownMenuItem
                                onClick={() => handleEditCIType(ciType)}
                                className="cursor-pointer"
                              >
                                <Edit className="mr-2 h-4 w-4" />
                                Edit
                              </DropdownMenuItem>
                              <DropdownMenuSeparator />
                              <DropdownMenuItem
                                onClick={() => handleDeleteCIType(ciType)}
                                className="text-red-600 cursor-pointer"
                              >
                                <Trash2 className="mr-2 h-4 w-4" />
                                Delete
                              </DropdownMenuItem>
                            </DropdownMenuContent>
                          </DropdownMenu>
                        </TableCell>
                      </TableRow>
                    );
                  })}
                </TableBody>
              </Table>
            </div>
          )}
        </CardContent>
      </Card>

      <CITypeForm
        open={isFormOpen}
        onOpenChange={setIsFormOpen}
        ciType={editingCIType}
        onSubmit={handleFormSubmit}
        isLoading={isSubmitting}
      />

      <Dialog open={!!deletingCIType} onOpenChange={() => !isSubmitting && setDeletingCIType(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete CI Type</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete &quot;{deletingCIType?.name}&quot;? This action cannot be undone.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => !isSubmitting && setDeletingCIType(null)}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button
              variant="destructive"
              onClick={confirmDelete}
              disabled={isSubmitting}
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Deleting...
                </>
              ) : (
                'Delete CI Type'
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
