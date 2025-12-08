import React, { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { ciTypeSchema, CITypeFormData } from '@/lib/validations';
import { CIType } from '@/lib/types';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@/components/ui/label';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Badge } from '@/components/ui/badge';
import { Loader2, Plus, X, Code } from 'lucide-react';

interface CITypeFormProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  ciType?: CIType | null;
  onSubmit: (data: CITypeFormData) => Promise<void>;
  isLoading?: boolean;
}

interface AttributeField {
  key: string;
  value: string;
  type: 'string' | 'number' | 'boolean' | 'json';
}

export function CITypeForm({ open, onOpenChange, ciType, onSubmit, isLoading }: CITypeFormProps) {
  const [attributes, setAttributes] = useState<AttributeField[]>(
    ciType?.attributes ?
      Object.entries(ciType.attributes).map(([key, value]) => ({
        key,
        value: typeof value === 'object' ? JSON.stringify(value) : String(value),
        type: typeof value === 'object' ? 'json' : typeof value === 'number' ? 'number' : typeof value === 'boolean' ? 'boolean' : 'string'
      })) : []
  );

  const form = useForm<CITypeFormData>({
    resolver: zodResolver(ciTypeSchema),
    defaultValues: {
      name: ciType?.name || '',
      description: ciType?.description || '',
      icon: ciType?.icon || '',
      color: ciType?.color || '#3b82f6',
      attributes: ciType?.attributes || {},
    },
  });

  React.useEffect(() => {
    if (ciType) {
      form.reset({
        name: ciType.name,
        description: ciType.description || '',
        icon: ciType.icon || '',
        color: ciType.color || '#3b82f6',
        attributes: ciType.attributes || {},
      });
      setAttributes(
        Object.entries(ciType.attributes || {}).map(([key, value]) => ({
          key,
          value: typeof value === 'object' ? JSON.stringify(value) : String(value),
          type: typeof value === 'object' ? 'json' : typeof value === 'number' ? 'number' : typeof value === 'boolean' ? 'boolean' : 'string'
        }))
      );
    } else {
      form.reset({
        name: '',
        description: '',
        icon: '',
        color: '#3b82f6',
        attributes: {},
      });
      setAttributes([]);
    }
  }, [ciType, form]);

  const addAttribute = () => {
    setAttributes([...attributes, { key: '', value: '', type: 'string' }]);
  };

  const removeAttribute = (index: number) => {
    const newAttributes = attributes.filter((_, i) => i !== index);
    setAttributes(newAttributes);
    updateAttributesObject(newAttributes);
  };

  const updateAttribute = (index: number, field: keyof AttributeField, value: string) => {
    const newAttributes = [...attributes];
    newAttributes[index] = { ...newAttributes[index], [field]: value };

    // Auto-detect type when value changes
    if (field === 'value') {
      if (value === '') {
        newAttributes[index].type = 'string';
      } else if (!isNaN(Number(value))) {
        newAttributes[index].type = 'number';
      } else if (value === 'true' || value === 'false') {
        newAttributes[index].type = 'boolean';
      } else if ((value.startsWith('{') && value.endsWith('}')) || (value.startsWith('[') && value.endsWith(']'))) {
        newAttributes[index].type = 'json';
      } else {
        newAttributes[index].type = 'string';
      }
    }

    setAttributes(newAttributes);
    updateAttributesObject(newAttributes);
  };

  const updateAttributesObject = (attrs: AttributeField[]) => {
    const attributesObj: Record<string, any> = {};
    attrs.forEach((attr) => {
      if (attr.key && attr.value) {
        let parsedValue: any = attr.value;

        switch (attr.type) {
          case 'number':
            parsedValue = Number(attr.value);
            break;
          case 'boolean':
            parsedValue = attr.value === 'true';
            break;
          case 'json':
            try {
              parsedValue = JSON.parse(attr.value);
            } catch {
              // If invalid JSON, keep as string
              parsedValue = attr.value;
            }
            break;
          default:
            parsedValue = attr.value;
        }

        attributesObj[attr.key] = parsedValue;
      }
    });
    form.setValue('attributes', attributesObj);
  };

  const handleFormSubmit = async (data: CITypeFormData) => {
    try {
      await onSubmit(data);
      onOpenChange(false);
      form.reset();
      setAttributes([]);
    } catch (error) {
      // Error is handled by the parent component
    }
  };

  const handleClose = () => {
    if (!isLoading) {
      onOpenChange(false);
      form.reset();
      setAttributes([]);
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {ciType ? 'Edit CI Type' : 'Create CI Type'}
          </DialogTitle>
          <DialogDescription>
            {ciType
              ? 'Update the configuration for this CI type.'
              : 'Define a new CI type with its attributes and properties.'
            }
          </DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form onSubmit={form.handleSubmit(handleFormSubmit)} className="space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <FormField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Name *</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="e.g., Server, Database, Application"
                        {...field}
                        disabled={isLoading}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="icon"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Icon</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="e.g., server, database, laptop"
                        {...field}
                        disabled={isLoading}
                      />
                    </FormControl>
                    <FormDescription>
                      Icon name for visual representation
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="Describe this CI type and its purpose..."
                      className="resize-none"
                      rows={3}
                      {...field}
                      disabled={isLoading}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="color"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Color</FormLabel>
                  <div className="flex items-center space-x-2">
                    <FormControl>
                      <Input
                        type="color"
                        className="w-16 h-10 border border-input rounded cursor-pointer"
                        {...field}
                        disabled={isLoading}
                      />
                    </FormControl>
                    <Input
                      value={field.value}
                      onChange={(e) => field.onChange(e.target.value)}
                      placeholder="#3b82f6"
                      className="flex-1"
                      disabled={isLoading}
                    />
                    <div
                      className="w-10 h-10 rounded border border-input"
                      style={{ backgroundColor: field.value }}
                    />
                  </div>
                  <FormDescription>
                    Color for UI representation and visual identification
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <FormLabel className="text-base font-medium">Attributes</FormLabel>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={addAttribute}
                  disabled={isLoading}
                >
                  <Plus className="w-4 h-4 mr-2" />
                  Add Attribute
                </Button>
              </div>

              <div className="space-y-3">
                {attributes.map((attr, index) => (
                  <div key={index} className="flex items-center space-x-2">
                    <Input
                      placeholder="Attribute name"
                      value={attr.key}
                      onChange={(e) => updateAttribute(index, 'key', e.target.value)}
                      className="flex-1"
                      disabled={isLoading}
                    />
                    <Input
                      placeholder="Value"
                      value={attr.value}
                      onChange={(e) => updateAttribute(index, 'value', e.target.value)}
                      className="flex-1"
                      disabled={isLoading}
                    />
                    <Badge variant="secondary" className="capitalize">
                      {attr.type}
                    </Badge>
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon"
                      onClick={() => removeAttribute(index)}
                      disabled={isLoading}
                      className="text-red-500 hover:text-red-700"
                    >
                      <X className="w-4 h-4" />
                    </Button>
                  </div>
                ))}
                {attributes.length === 0 && (
                  <div className="text-center py-6 text-muted-foreground border-2 border-dashed border-muted-foreground/25 rounded-lg">
                    <Code className="w-8 h-8 mx-auto mb-2 opacity-50" />
                    <p>No attributes defined</p>
                    <p className="text-sm">Click &quot;Add Attribute&quot; to define custom fields</p>
                  </div>
                )}
              </div>

              <FormDescription>
                Define custom attributes for this CI type. These can be used to store additional metadata specific to your infrastructure.
              </FormDescription>
            </div>

            <DialogFooter>
              <Button
                type="button"
                variant="outline"
                onClick={handleClose}
                disabled={isLoading}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={isLoading}>
                {isLoading ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    {ciType ? 'Updating...' : 'Creating...'}
                  </>
                ) : (
                  ciType ? 'Update CI Type' : 'Create CI Type'
                )}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}