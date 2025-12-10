# Fix API Implementation Mismatches - Implementation Plan

## 🎯 Status: CRITICAL FIXES COMPLETED ✅

**Last Updated**: 2025-12-10
**Progress**: ~70% Complete - Core API fixes done, cleanup tasks remaining

## Context

**Project**: Crate IT Asset Management Platform
**Implemented**: Phase 1 (Foundation) + Phase 2.1 (CI Types) + Phase 2.2 (Lifecycles)
**Issue**: Critical API mismatches between frontend and backend causing 404 errors

## ✅ What's Been Fixed

### Phase 1: Base URL Configuration ✅ COMPLETE
- Updated `.env.local.example` with `/api/v1` prefix
- Updated `api.ts` default base URL to include `/api/v1`
- **Result**: All API calls now use correct base URL

### Phase 2: Endpoint Path Mismatches ✅ COMPLETE
- Fixed all CI Types endpoints (ci/types → ci-types)
- Fixed all Lifecycles endpoints (ci/lifecycles → lifecycle-types)
- Fixed all Assets endpoints (ci/assets → ci-assets)
- Fixed all Relationship Types endpoints (ci/relationship-types → relationship-types)
- Replaced all `get().apiRequest()` calls with `apiClient` methods
- **Result**: All store methods now use correct paths and centralized API client

### Phase 3: Remove Custom apiRequest ✅ COMPLETE
- Deleted custom `apiRequest` method from ci-store.ts
- Removed from interface definition
- **Result**: Single, centralized API client throughout codebase

### Phase 4: Replace Hardcoded Fetch Calls (PARTIAL)
- ✅ `assets/page.tsx` - All 5 fetch calls replaced with apiClient
- ⏳ `lifecycles/page.tsx` - Not started (similar pattern to assets)
- ⏳ `relationships/page.tsx` - Not started (similar pattern to assets)

## Root Cause Analysis

The backend nests ALL routes under `/api/v1` prefix (backend/src/main.rs:156), but the frontend API configuration doesn't account for this, causing all API calls to fail with 404 errors.

### Critical Issues Found

1. **Base URL Missing `/api/v1` Prefix** - ALL API calls fail
2. **Inconsistent Endpoint Naming** - Frontend uses `/ci/types`, backend has `/ci-types`
3. **Multiple API Request Patterns** - Three different methods used inconsistently
4. **Missing Backend Endpoints** - Relationships CRUD not implemented (only relationship types exist)
5. **Hardcoded Fetch Calls** - Components bypass centralized API client

## Implementation Strategy

**User's Choice**: Add `/api/v1` to `NEXT_PUBLIC_API_URL` in frontend configuration

---

## Phase 1: Fix Base URL Configuration (CRITICAL - Priority 1)

### 1.1 Update Environment Configuration

**File**: `frontend/.env.local.example` (Line 2)
```diff
- NEXT_PUBLIC_API_URL=http://localhost:8080
+ NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
```

**File**: `frontend/src/lib/api.ts` (Line 4)
```diff
- const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';
+ const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api/v1';
```

**Action**: If `.env.local` exists, update it manually to include `/api/v1`

---

## Phase 2: Fix Endpoint Path Mismatches (CRITICAL - Priority 1)

### 2.1 CI Types Endpoints

**File**: `frontend/src/store/ci-store.ts`

**Lines to Fix**:
- Line 114: `/ci/types` → `/ci-types`
- Line 126: `/ci/types` → `/ci-types` + replace `apiRequest` with `apiClient.post`
- Line 135: `/ci/types/${id}` → `/ci-types/${id}` + replace `apiRequest` with `apiClient.put`
- Line 146: `/ci/types/${id}` → `/ci-types/${id}` + replace `apiRequest` with `apiClient.delete`

**Changes**:
```typescript
// Line 114
const data = await apiClient.get('/ci-types') as CIType[];

// Line 126 (createCIType)
const result = await apiClient.post('/ci-types', data);

// Line 135 (updateCIType)
const result = await apiClient.put(`/ci-types/${id}`, data);

// Line 146 (deleteCIType)
await apiClient.delete(`/ci-types/${id}`);
```

### 2.2 Lifecycle Endpoints

**File**: `frontend/src/store/ci-store.ts`

**Lines to Fix**:
- Line 156: `/ci/lifecycles` → `/lifecycle-types`
- Lines 167-188: Replace all `/ci/lifecycles` with `/lifecycle-types` and use `apiClient`

**Changes**:
```typescript
// Line 156
const data = await apiClient.get('/lifecycle-types');

// Similar pattern for create, update, delete using apiClient methods
```

### 2.3 Assets Endpoints

**File**: `frontend/src/store/ci-store.ts`

**Lines to Fix**:
- Line 206: `/ci/assets` → `/ci-assets`
- Lines 218-242: Replace all `/ci/assets` with `/ci-assets` and use `apiClient`

**Changes**:
```typescript
// Line 206
const endpoint = `/ci-assets${params.toString() ? `?${params.toString()}` : ''}`;
const data = await apiClient.get(endpoint);

// Similar pattern for create, update, delete
```

### 2.4 Relationship Types Endpoints

**File**: `frontend/src/store/ci-store.ts`

**Lines to Fix**:
- Line 253: `/ci/relationship-types` → `/relationship-types`
- Lines 264-288: Use `/relationship-types` and `apiClient`

**Changes**:
```typescript
// Line 253
const data = await apiClient.get('/relationship-types');

// Similar pattern for create, update, delete
```

---

## Phase 3: Remove Custom apiRequest Method (Priority 1)

### 3.1 Delete apiRequest Implementation

**File**: `frontend/src/store/ci-store.ts`

**Remove**:
- Lines 27-29: Remove `apiRequest` from interface
- Lines 88-108: Delete entire `apiRequest` method implementation

**Reason**: This method hardcodes `/api` prefix and duplicates `apiClient` functionality. All calls should use the centralized `apiClient` instead.

---

## Phase 4: Fix Hardcoded Fetch Calls (Priority 2)

### 4.1 CI Assets Page

**File**: `frontend/src/app/ci-management/assets/page.tsx`

**Add Import** (top of file):
```typescript
import { apiClient } from '@/lib/api';
```

**Replace All `fetch()` Calls**:
- Line 98: Fetch assets
- Line 116: Fetch CI types
- Line 216: Update asset (PUT)
- Line 238: Create asset (POST)
- Line 290: Delete asset (DELETE)

**Pattern**:
```typescript
// BEFORE:
const response = await fetch(`/api/v1/ci-assets?${params}`);
const data = await response.json();

// AFTER:
const data = await apiClient.get(`/ci-assets?${params}`);
```

**Add try-catch for error handling**:
```typescript
try {
  const data = await apiClient.get(`/ci-assets?${params}`);
  setAssets(data.data || []);
} catch (error) {
  toast.error(error instanceof Error ? error.message : 'Failed to fetch assets');
}
```

### 4.2 Lifecycles Page

**File**: `frontend/src/app/ci-management/lifecycles/page.tsx`

**Add Import**:
```typescript
import { apiClient } from '@/lib/api';
```

**Replace fetch calls** (Lines 100, 119, 139, 176, 207, 232, 272):
- Use `apiClient.get()`, `apiClient.post()`, `apiClient.put()`, `apiClient.delete()`
- Wrap in try-catch blocks

### 4.3 Relationships Page

**File**: `frontend/src/app/ci-management/relationships/page.tsx`

**Add Import**:
```typescript
import { apiClient } from '@/lib/api';
```

**Replace fetch calls** (Lines 65, 83, 126, 148, 208):
- Use `apiClient` methods
- Wrap in try-catch blocks

---

## Phase 5: Handle Missing Backend Endpoints (Priority 3)

### 5.1 Remove Relationships CRUD from CI Store

**File**: `frontend/src/store/ci-store.ts`

**Remove These Methods** (Lines 292-332):
- `fetchRelationships`
- `createRelationship`
- `updateRelationship`
- `deleteRelationship`

**Also Remove from Interface** (Lines 56-60):
```typescript
// REMOVE these lines:
fetchRelationships: (ciAssetId?: string) => Promise<void>;
createRelationship: (data: Partial<Relationship>) => Promise<Relationship>;
updateRelationship: (id: string, data: Partial<Relationship>) => Promise<Relationship>;
deleteRelationship: (id: string) => Promise<void>;
```

**Also Remove from State**:
- `relationships: Relationship[]` from state
- `relationships: boolean` from loading state
- `relationships?: string` from errors state

**Reason**: Backend only implements Relationship **Types** CRUD, not actual Relationship instances. This is Phase 3.1 work (not yet implemented).

### 5.2 Clean Up API Endpoints

**File**: `frontend/src/lib/api.ts`

**Update apiEndpoints object** (Lines 159-206):
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

  // Graph endpoints are stubs - keep but note they're placeholders
  graph: {
    data: '/graph/data',
    // layout: '/graph/layout',  // TODO: Not implemented
    search: '/graph/search',
  },

  audit: {
    logs: '/audit/logs',
    // changes: '/audit/changes',  // TODO: Not implemented
  },

  dashboard: {
    stats: '/dashboard/stats',
    // activity: '/dashboard/activity',  // TODO: Not implemented
    // topAssets: '/dashboard/top-assets',  // TODO: Not implemented
  },

  // Amortization endpoints
  // amortization: {
  //   records: '/amortization/records',
  //   schedule: '/amortization/assets/:id/schedule',
  // },
} as const;
```

---

## Phase 6: Update Documentation (Priority 4)

### 6.1 Update README and Setup Docs

**Files**:
- `README.md` (Lines 166-189)
- `docs/development/setup.md`
- `CLAUDE.md` (if exists)

**Update all references** to `NEXT_PUBLIC_API_URL` to include `/api/v1`:
```env
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
```

---

## Implementation Checklist

### Critical (Must Do First) ✅ **COMPLETED**
- [x] Update `frontend/.env.local.example` with `/api/v1`
- [x] Update `frontend/src/lib/api.ts` default base URL
- [ ] Create/update `.env.local` with correct URL (if not already exists) **← USER ACTION REQUIRED**
- [x] Fix CI Types paths in `ci-store.ts` (ci/types → ci-types)
- [x] Fix Lifecycles paths in `ci-store.ts` (ci/lifecycles → lifecycle-types)
- [x] Fix Assets paths in `ci-store.ts` (ci/assets → ci-assets)
- [x] Fix Relationship Types paths in `ci-store.ts`
- [x] Remove `apiRequest` method from `ci-store.ts`

### High Priority
- [x] Replace hardcoded fetch in `assets/page.tsx` **← COMPLETED**
- [ ] Replace hardcoded fetch in `lifecycles/page.tsx` **← REMAINING**
- [ ] Replace hardcoded fetch in `relationships/page.tsx` **← REMAINING**
- [x] Add proper error handling with try-catch blocks (done for assets page)

### Medium Priority
- [ ] Remove relationships CRUD from `ci-store.ts` **← REMAINING**
- [ ] Clean up `apiEndpoints` in `api.ts` **← REMAINING**
- [ ] Remove unused state properties

### Low Priority
- [ ] Update documentation files (README.md, CLAUDE.md)
- [ ] Test all flows end-to-end
- [ ] Verify network requests in DevTools

---

## Testing Strategy

After implementation, verify:

1. **Login Flow**
   - Visit http://localhost:3000/auth/login
   - Login with credentials
   - Token stored in localStorage
   - Redirected to dashboard

2. **CI Types**
   - List CI types (GET /api/v1/ci-types)
   - Create new CI type (POST /api/v1/ci-types)
   - Update CI type (PUT /api/v1/ci-types/:id)
   - Delete CI type (DELETE /api/v1/ci-types/:id)

3. **Lifecycles**
   - List lifecycle types (GET /api/v1/lifecycle-types)
   - Create lifecycle (POST /api/v1/lifecycle-types)
   - Add states (POST /api/v1/lifecycle-states)
   - Update/delete operations

4. **Assets**
   - List assets (GET /api/v1/ci-assets)
   - Filter by CI type
   - Create/update/delete assets

5. **Relationship Types**
   - List relationship types
   - Create/update/delete operations

6. **DevTools Network Tab**
   - All requests go to: `http://localhost:8080/api/v1/...`
   - All requests include: `Authorization: Bearer <token>`
   - No 404 errors
   - Successful responses (200, 201, 204)

---

## Critical Files to Modify

**Priority 1 (Breaks Everything)**:
1. `frontend/.env.local.example:2`
2. `frontend/src/lib/api.ts:4`
3. `frontend/src/store/ci-store.ts:114,126,135,146,156,167-188,206,218-242,253,264-288`
4. `frontend/src/store/ci-store.ts:27-29,88-108` (delete apiRequest)

**Priority 2 (Code Quality)**:
5. `frontend/src/app/ci-management/assets/page.tsx:98,116,216,238,290`
6. `frontend/src/app/ci-management/lifecycles/page.tsx:100,119,139,176,207,232,272`
7. `frontend/src/app/ci-management/relationships/page.tsx:65,83,126,148,208`

**Priority 3 (Cleanup)**:
8. `frontend/src/store/ci-store.ts:292-332` (remove relationships CRUD)
9. `frontend/src/lib/api.ts:159-206` (clean up endpoints)

**Priority 4 (Documentation)**:
10. `README.md`, `docs/development/setup.md`, `CLAUDE.md`

---

## Expected Outcome

After implementation:
- ✅ All API calls use correct base URL: `http://localhost:8080/api/v1/...`
- ✅ All endpoint paths match backend routes (ci-types, lifecycle-types, etc.)
- ✅ Single, centralized API client used throughout frontend
- ✅ No hardcoded fetch calls bypassing API client
- ✅ Authentication flows work correctly
- ✅ All CRUD operations for CI Types, Lifecycles, Assets, Relationship Types work
- ✅ No 404 errors in browser console
- ✅ Clean, maintainable codebase ready for Phase 2.3 and beyond

---

## Notes

- Backend runs on port **8080** (configurable)
- Frontend dev server on port **3000**
- Backend routes ALL nested under `/api/v1` (main.rs:156)
- Relationship **instances** CRUD is Phase 3.1 work (not yet implemented)
- Current implementation: Phases 1, 2.1, 2.2 complete
- Next phases: 2.3 (Relationship Types - **already implemented in backend**), 2.4 (Assets - **partially implemented**)
