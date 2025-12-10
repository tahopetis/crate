# API Fix Completion Guide

## ✅ What's Been Completed (70%)

### Critical Fixes - ALL DONE ✅
1. **Base URL Configuration** - Added `/api/v1` prefix to frontend config
2. **CI Store Endpoints** - Fixed all endpoint paths and replaced apiRequest with apiClient
3. **Assets Page** - Replaced all hardcoded fetch calls with apiClient

## 📋 Remaining Tasks (30%)

### Task 1: Update .env.local File
**Priority**: CRITICAL - Required for app to work
**Location**: `frontend/.env.local`
**Action**: Create or update this file with:
```env
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
```

### Task 2: Replace Hardcoded Fetch in Lifecycles Page
**Priority**: High
**File**: `frontend/src/app/ci-management/lifecycles/page.tsx`
**Lines**: 100, 119, 139, 176, 207, 232, 272

**Pattern to Follow** (same as assets page):
```typescript
// Add import at top
import { apiClient } from '@/lib/api';

// Replace fetch calls
// BEFORE:
const response = await fetch('/api/v1/lifecycle-types');
const data = await response.json();
if (data.success) { ... }

// AFTER:
const data = await apiClient.get('/lifecycle-types');
setCiTypes(data.data || []);
```

**Specific Changes Needed**:
- Line ~100: `fetch('/api/v1/lifecycle-types')` → `apiClient.get('/lifecycle-types')`
- Line ~119: `fetch('/api/v1/lifecycle-types/:id')` → `apiClient.get('/lifecycle-types/:id')`
- Line ~139: `fetch('/api/v1/lifecycle-types', {method: 'POST'})` → `apiClient.post('/lifecycle-types', data)`
- Line ~176: `fetch(..., {method: 'PUT'})` → `apiClient.put('/lifecycle-types/:id', data)`
- Line ~207: `fetch(..., {method: 'DELETE'})` → `apiClient.delete('/lifecycle-types/:id')`
- Similar for lifecycle-states endpoints

### Task 3: Replace Hardcoded Fetch in Relationships Page
**Priority**: High
**File**: `frontend/src/app/ci-management/relationships/page.tsx`
**Lines**: 65, 83, 126, 148, 208

**Follow same pattern as Task 2**

### Task 4: Remove Relationships CRUD Methods
**Priority**: Medium
**File**: `frontend/src/store/ci-store.ts`
**Lines to Remove**: 292-332 (and interface definitions at 56-60)

**Why**: Backend only implements Relationship **Types**, not Relationship instances. Phase 3.1 will add these.

**Methods to Remove**:
```typescript
// Remove these from interface (lines 56-60):
fetchRelationships: (ciAssetId?: string) => Promise<void>;
createRelationship: (data: Partial<Relationship>) => Promise<Relationship>;
updateRelationship: (id: string, data: Partial<Relationship>) => Promise<Relationship>;
deleteRelationship: (id: string) => Promise<void>;

// Remove these from implementation (lines 292-332):
fetchRelationships: async (ciAssetId) => { ... },
createRelationship: async (data) => { ... },
updateRelationship: async (id, data) => { ... },
deleteRelationship: async (id) => { ... },
```

**Also Remove from State**:
- Line 11: `relationships: Relationship[];`
- Line 18: `relationships: boolean;` (in loading)
- Line 25: `relationships?: string;` (in errors)

### Task 5: Clean Up API Endpoints
**Priority**: Medium
**File**: `frontend/src/lib/api.ts`
**Lines**: 159-206

**Changes**:
```typescript
export const apiEndpoints = {
  auth: {
    login: '/auth/login',
    register: '/auth/register',
    logout: '/auth/logout',
    me: '/auth/me',
    // refresh: '/auth/refresh',  // TODO: Not implemented in backend
  },

  ci: {
    types: '/ci-types',
    lifecycles: '/lifecycle-types',
    assets: '/ci-assets',
    relationshipTypes: '/relationship-types',
    // relationships: '/relationships',  // TODO: Phase 3.1 - Not implemented yet
    import: '/import/ci-assets',
    export: '/export/ci-assets',
  },

  // Comment out unimplemented endpoints
  // graph: { ... },  // TODO: Implement in backend

  audit: {
    logs: '/audit/logs',
    // changes: '/audit/changes',  // TODO: Not implemented
  },

  dashboard: {
    stats: '/dashboard/stats',
    // activity: '/dashboard/activity',  // TODO: Not implemented
    // topAssets: '/dashboard/top-assets',  // TODO: Not implemented
  },
} as const;
```

### Task 6: Update Documentation
**Priority**: Low
**Files**:
- `README.md` (lines 166-189)
- `CLAUDE.md` (if exists)

**Change**: Update all references to `NEXT_PUBLIC_API_URL` to include `/api/v1`:
```env
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
```

## 🧪 Testing After Completion

### Quick Test
1. Create `.env.local` with correct URL
2. Restart frontend dev server: `cd frontend && pnpm dev`
3. Open browser DevTools → Network tab
4. Test basic flows:
   - Login
   - View CI Types
   - Create CI Type
   - View Assets

### Verify in Network Tab
All requests should show:
- URL: `http://localhost:8080/api/v1/...`
- Status: 200, 201, or 204 (not 404!)
- Headers include: `Authorization: Bearer <token>`

## 📝 Quick Reference

### Files Modified So Far
1. ✅ `frontend/.env.local.example:2` - Added /api/v1
2. ✅ `frontend/src/lib/api.ts:4` - Added /api/v1 to default
3. ✅ `frontend/src/store/ci-store.ts` - Fixed all endpoints, removed apiRequest
4. ✅ `frontend/src/app/ci-management/assets/page.tsx` - Replaced fetch with apiClient

### Files Still To Modify
5. ⏳ `frontend/.env.local` - **USER MUST CREATE/UPDATE**
6. ⏳ `frontend/src/app/ci-management/lifecycles/page.tsx`
7. ⏳ `frontend/src/app/ci-management/relationships/page.tsx`
8. ⏳ `frontend/src/store/ci-store.ts` - Remove relationships CRUD
9. ⏳ `frontend/src/lib/api.ts` - Clean up endpoints
10. ⏳ Documentation files

## 🎯 Next Steps

1. **CRITICAL**: Create `frontend/.env.local` file with correct API URL
2. Complete remaining fetch replacements in lifecycles and relationships pages
3. Remove unimplemented relationships CRUD methods
4. Test the application
5. Update documentation

## 💡 Need Help?

Refer to:
- Full plan: `docs/api-mismatch-fix-plan.md`
- Implementation patterns from `assets/page.tsx` (already completed)
- Original analysis document for background context
