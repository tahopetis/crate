import { z } from 'zod';

// Common validation schemas
export const emailSchema = z
  .string()
  .min(1, 'Email is required')
  .email('Invalid email address');

export const passwordSchema = z
  .string()
  .min(8, 'Password must be at least 8 characters long')
  .regex(/[A-Z]/, 'Password must contain at least one uppercase letter')
  .regex(/[a-z]/, 'Password must contain at least one lowercase letter')
  .regex(/\d/, 'Password must contain at least one number')
  .regex(/[!@#$%^&*(),.?":{}|<>]/, 'Password must contain at least one special character');

export const nameSchema = z
  .string()
  .min(1, 'Name is required')
  .max(100, 'Name must be less than 100 characters')
  .regex(/^[a-zA-Z\s'-]+$/, 'Name can only contain letters, spaces, hyphens, and apostrophes');

export const requiredStringSchema = z
  .string()
  .min(1, 'This field is required');

export const optionalStringSchema = z.string().optional();

export const numberSchema = z
  .number({
    required_error: 'This field is required',
    invalid_type_error: 'Must be a number',
  })
  .min(0, 'Must be a positive number');

export const optionalNumberSchema = z.number().optional();

export const dateSchema = z
  .string()
  .min(1, 'Date is required')
  .datetime('Invalid date format');

export const optionalDateSchema = z.string().datetime().optional();

// Authentication schemas
export const loginSchema = z.object({
  email: emailSchema,
  password: z.string().min(1, 'Password is required'),
});

export const registerSchema = z.object({
  name: nameSchema,
  email: emailSchema,
  password: passwordSchema,
  confirmPassword: z.string().min(1, 'Please confirm your password'),
}).refine((data) => data.password === data.confirmPassword, {
  message: "Passwords don't match",
  path: ["confirmPassword"],
});

export type LoginFormData = z.infer<typeof loginSchema>;
export type RegisterFormData = z.infer<typeof registerSchema>;

// CI Type schemas
export const ciTypeSchema = z.object({
  name: requiredStringSchema.max(100, 'Name must be less than 100 characters'),
  description: optionalStringSchema.max(500, 'Description must be less than 500 characters'),
  icon: optionalStringSchema.max(50, 'Icon name must be less than 50 characters'),
  color: optionalStringSchema.regex(/^#[0-9A-F]{6}$/i, 'Invalid color format'),
  attributes: z.record(z.any()).optional().default({}),
});

export type CITypeFormData = z.infer<typeof ciTypeSchema>;

// CI Asset schemas
export const ciAssetSchema = z.object({
  name: requiredStringSchema.max(200, 'Name must be less than 200 characters'),
  ci_type_id: requiredStringSchema,
  lifecycle_status_id: requiredStringSchema,
  attributes: z.record(z.any()).optional().default({}),
  value: optionalNumberSchema,
  purchase_date: optionalDateSchema,
  depreciation_period: z.number().int().min(1).max(600).optional(), // 1 month to 50 years
});

export type CIAssetFormData = z.infer<typeof ciAssetSchema>;

// Lifecycle schemas
export const lifecycleSchema = z.object({
  name: requiredStringSchema.max(100, 'Name must be less than 100 characters'),
  description: optionalStringSchema.max(500, 'Description must be less than 500 characters'),
  statuses: z.array(z.object({
    name: requiredStringSchema.max(100, 'Status name must be less than 100 characters'),
    order: z.number().int().min(0),
    color: optionalStringSchema.regex(/^#[0-9A-F]{6}$/i, 'Invalid color format'),
    description: optionalStringSchema.max(500, 'Description must be less than 500 characters'),
  })).min(1, 'At least one status is required'),
});

export type LifecycleFormData = z.infer<typeof lifecycleSchema>;

// Relationship Type schemas
export const relationshipTypeSchema = z.object({
  name: requiredStringSchema.max(100, 'Name must be less than 100 characters'),
  description: optionalStringSchema.max(500, 'Description must be less than 500 characters'),
  is_bidirectional: z.boolean().default(false),
  source_cardinality: z.enum(['one', 'many'], {
    required_error: 'Source cardinality is required',
  }),
  target_cardinality: z.enum(['one', 'many'], {
    required_error: 'Target cardinality is required',
  }),
});

export type RelationshipTypeFormData = z.infer<typeof relationshipTypeSchema>;

// Relationship schemas
export const relationshipSchema = z.object({
  relationship_type_id: requiredStringSchema,
  source_ci_id: requiredStringSchema,
  target_ci_id: requiredStringSchema,
  attributes: z.record(z.any()).optional().default({}),
}).refine((data) => data.source_ci_id !== data.target_ci_id, {
  message: 'Source and target CI cannot be the same',
  path: ['target_ci_id'],
});

export type RelationshipFormData = z.infer<typeof relationshipSchema>;

// Filter schemas
export const ciFiltersSchema = z.object({
  ci_type_id: optionalStringSchema,
  lifecycle_status_id: optionalStringSchema,
  search: optionalStringSchema,
  value_min: optionalNumberSchema,
  value_max: optionalNumberSchema,
  purchase_date_from: optionalDateSchema,
  purchase_date_to: optionalDateSchema,
});

export type CIFiltersData = z.infer<typeof ciFiltersSchema>;

export const auditFiltersSchema = z.object({
  action: z.enum(['create', 'update', 'delete']).optional(),
  entity_type: z.enum(['ci_type', 'ci_asset', 'relationship', 'lifecycle']).optional(),
  user_id: optionalStringSchema,
  date_from: optionalDateSchema,
  date_to: optionalDateSchema,
  search: optionalStringSchema,
});

export type AuditFiltersData = z.infer<typeof auditFiltersSchema>;

// Pagination schemas
export const paginationSchema = z.object({
  page: z.coerce.number().int().min(1).default(1),
  limit: z.coerce.number().int().min(1).max(100).default(10),
  sortBy: optionalStringSchema,
  sortOrder: z.enum(['asc', 'desc']).default('desc'),
});

export type PaginationData = z.infer<typeof paginationSchema>;

// Search schema
export const searchSchema = z.object({
  query: z.string().min(1, 'Search query is required').max(100, 'Search query is too long'),
  filters: z.record(z.any()).optional(),
});

export type SearchData = z.infer<typeof searchSchema>;

// File upload schema
export const fileUploadSchema = z.object({
  file: z.instanceof(File).refine(
    (file) => file.size <= 10 * 1024 * 1024, // 10MB
    'File size must be less than 10MB'
  ).refine(
    (file) => ['text/csv', 'application/vnd.ms-excel', 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'].includes(file.type),
    'Only CSV and Excel files are allowed'
  ),
  description: optionalStringSchema,
});

export type FileUploadData = z.infer<typeof fileUploadSchema>;

// Form validation utilities
export const validateForm = <T extends z.ZodSchema>(
  schema: T,
  data: unknown
): { success: true; data: z.infer<T> } | { success: false; errors: Record<string, string> } => {
  const result = schema.safeParse(data);

  if (result.success) {
    return { success: true, data: result.data };
  }

  const errors: Record<string, string> = {};
  result.error.issues.forEach((issue) => {
    const path = issue.path.join('.');
    errors[path] = issue.message;
  });

  return { success: false, errors };
};

// Custom validation messages
export const validationMessages = {
  required: 'This field is required',
  email: 'Please enter a valid email address',
  password: {
    length: 'Password must be at least 8 characters long',
    uppercase: 'Password must contain at least one uppercase letter',
    lowercase: 'Password must contain at least one lowercase letter',
    number: 'Password must contain at least one number',
    special: 'Password must contain at least one special character',
  },
  name: 'Please enter a valid name',
  number: 'Please enter a valid number',
  date: 'Please enter a valid date',
  color: 'Please enter a valid color (hex format)',
  fileSize: 'File size must be less than 10MB',
  fileType: 'Only CSV and Excel files are allowed',
  select: 'Please select an option',
  url: 'Please enter a valid URL',
  phone: 'Please enter a valid phone number',
  postalCode: 'Please enter a valid postal code',
} as const;