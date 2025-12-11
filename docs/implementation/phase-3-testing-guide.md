# Phase 3 Testing Guide: Graph Visualization

## Testing Checklist

This guide provides step-by-step instructions for testing the graph visualization feature with real data.

---

## Prerequisites

### 1. Start All Services

```bash
# Terminal 1: Start Docker services (PostgreSQL, Neo4j, Redis)
docker-compose up -d

# Terminal 2: Start backend
cd backend
cargo run

# Terminal 3: Start frontend
cd frontend
pnpm dev
```

### 2. Verify Services

- **Frontend**: http://localhost:3000
- **Backend**: http://localhost:8080
- **Neo4j Browser**: http://localhost:7474 (User: neo4j, Password: dev12345)
- **PostgreSQL**: Port 5432 (User: dev, Password: dev123, Database: crate_dev)

---

## Test Scenario 1: Basic Graph Visualization (Empty State)

### Steps:
1. Navigate to http://localhost:3000/auth/login
2. Log in with your credentials
3. Click "Graph" in the sidebar
4. **Expected Result**:
   - Page loads without errors
   - Shows "0 nodes" and "0 edges" in controls
   - Graph canvas displays empty state message
   - All controls are visible (zoom, pan, layout selector)
   - CI type filter shows "All CI Types"
   - Search box is present

### Verification:
- [ ] Page loads successfully
- [ ] No console errors in browser DevTools
- [ ] Empty state displays correctly
- [ ] All UI components are visible

---

## Test Scenario 2: Create Test Data

### Step 2.1: Create CI Types

Navigate to **CI Management → Types** and create the following types:

1. **Server**
   - Name: `Server`
   - Description: `Physical or virtual server`
   - Schema:
   ```json
   {
     "hostname": {"type": "string", "required": true},
     "ip_address": {"type": "string"},
     "os": {"type": "string"}
   }
   ```

2. **Database**
   - Name: `Database`
   - Description: `Database instance`
   - Schema:
   ```json
   {
     "db_type": {"type": "string", "required": true},
     "version": {"type": "string"},
     "port": {"type": "number"}
   }
   ```

3. **Application**
   - Name: `Application`
   - Description: `Software application`
   - Schema:
   ```json
   {
     "app_name": {"type": "string", "required": true},
     "version": {"type": "string"},
     "language": {"type": "string"}
   }
   ```

### Step 2.2: Create CI Assets

Navigate to **CI Management → Assets** and create:

**Servers:**
1. Name: `web-server-01`, Type: Server, Attributes: `{"hostname": "web01", "ip_address": "10.0.1.10", "os": "Ubuntu 22.04"}`
2. Name: `web-server-02`, Type: Server, Attributes: `{"hostname": "web02", "ip_address": "10.0.1.11", "os": "Ubuntu 22.04"}`
3. Name: `app-server-01`, Type: Server, Attributes: `{"hostname": "app01", "ip_address": "10.0.2.10", "os": "Ubuntu 22.04"}`

**Databases:**
1. Name: `postgres-primary`, Type: Database, Attributes: `{"db_type": "PostgreSQL", "version": "15.2", "port": 5432}`
2. Name: `redis-cache`, Type: Database, Attributes: `{"db_type": "Redis", "version": "7.0", "port": 6379}`

**Applications:**
1. Name: `ecommerce-api`, Type: Application, Attributes: `{"app_name": "E-Commerce API", "version": "2.1.0", "language": "Rust"}`
2. Name: `payment-service`, Type: Application, Attributes: `{"app_name": "Payment Service", "version": "1.5.0", "language": "Python"}`
3. Name: `frontend-app`, Type: Application, Attributes: `{"app_name": "Frontend", "version": "3.0.0", "language": "TypeScript"}`

### Step 2.3: Create Relationship Types

Navigate to **CI Management → Relationships** and create:

1. **Runs On**
   - Name: `Runs On`
   - Description: `Application runs on server`
   - From Type: `Application`
   - To Type: `Server`
   - Bidirectional: `false`

2. **Connects To**
   - Name: `Connects To`
   - Description: `Application connects to database`
   - From Type: `Application`
   - To Type: `Database`
   - Bidirectional: `false`

3. **Depends On**
   - Name: `Depends On`
   - Description: `Service dependency`
   - From Type: `Application`
   - To Type: `Application`
   - Bidirectional: `false`

### Step 2.4: Create Relationships (via API)

Use the following curl commands to create relationships:

```bash
# Get auth token first (replace with your actual credentials)
TOKEN=$(curl -s -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"your@email.com","password":"yourpassword"}' | jq -r '.data.token')

# Get IDs of created assets (you'll need these from the UI or database)
# For this guide, replace <UUID> placeholders with actual IDs

# Relationship 1: ecommerce-api runs on app-server-01
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<runs-on-type-id>",
    "from_ci_asset_id": "<ecommerce-api-id>",
    "to_ci_asset_id": "<app-server-01-id>",
    "attributes": {}
  }'

# Relationship 2: payment-service runs on app-server-01
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<runs-on-type-id>",
    "from_ci_asset_id": "<payment-service-id>",
    "to_ci_asset_id": "<app-server-01-id>",
    "attributes": {}
  }'

# Relationship 3: frontend-app runs on web-server-01
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<runs-on-type-id>",
    "from_ci_asset_id": "<frontend-app-id>",
    "to_ci_asset_id": "<web-server-01-id>",
    "attributes": {}
  }'

# Relationship 4: ecommerce-api connects to postgres-primary
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<connects-to-type-id>",
    "from_ci_asset_id": "<ecommerce-api-id>",
    "to_ci_asset_id": "<postgres-primary-id>",
    "attributes": {}
  }'

# Relationship 5: ecommerce-api connects to redis-cache
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<connects-to-type-id>",
    "from_ci_asset_id": "<ecommerce-api-id>",
    "to_ci_asset_id": "<redis-cache-id>",
    "attributes": {}
  }'

# Relationship 6: payment-service connects to postgres-primary
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<connects-to-type-id>",
    "from_ci_asset_id": "<payment-service-id>",
    "to_ci_asset_id": "<postgres-primary-id>",
    "attributes": {}
  }'

# Relationship 7: frontend-app depends on ecommerce-api
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<depends-on-type-id>",
    "from_ci_asset_id": "<frontend-app-id>",
    "to_ci_asset_id": "<ecommerce-api-id>",
    "attributes": {}
  }'

# Relationship 8: ecommerce-api depends on payment-service
curl -X POST http://localhost:8080/api/v1/relationships \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "relationship_type_id": "<depends-on-type-id>",
    "from_ci_asset_id": "<ecommerce-api-id>",
    "to_ci_asset_id": "<payment-service-id>",
    "attributes": {}
  }'
```

---

## Test Scenario 3: Graph Visualization with Data

### Steps:
1. Navigate to the Graph page (http://localhost:3000/graph)
2. Click the **Refresh** button

### Expected Results:
- [ ] Graph displays 8 nodes:
  - 3 Servers (blue circles)
  - 2 Databases (green circles)
  - 3 Applications (purple circles)
- [ ] Graph displays 8 edges (arrows between nodes)
- [ ] Controls show "8 nodes" and "8 edges"
- [ ] Nodes are positioned using force-directed layout (COSE)
- [ ] Edge labels show relationship types ("RUNS_ON", "CONNECTS_TO", "DEPENDS_ON")
- [ ] No console errors

### Visual Verification:
- Servers should be blue (#3B82F6)
- Databases should be green (#10B981)
- Applications should be purple (#8B5CF6)
- Edges should have arrows pointing in the correct direction
- Node labels should display asset names

---

## Test Scenario 4: Graph Controls

### Test 4.1: Zoom Controls
1. Click **Zoom In** button
   - **Expected**: Graph zooms in (nodes get larger)
2. Click **Zoom Out** button
   - **Expected**: Graph zooms out (nodes get smaller)
3. Use mouse wheel to zoom
   - **Expected**: Smooth zoom in/out

### Test 4.2: Pan and Fit
1. Click and drag on empty canvas
   - **Expected**: Graph pans (moves)
2. Click **Fit to Screen** button
   - **Expected**: Graph resizes to fit all nodes in viewport
3. Click **Reset View** button
   - **Expected**: Graph returns to original zoom and position

### Test 4.3: Layout Changes
1. Select **Circle** layout from dropdown
   - **Expected**: Nodes animate into circular arrangement
2. Select **Grid** layout
   - **Expected**: Nodes arrange in grid pattern
3. Select **Breadth First** layout
   - **Expected**: Nodes arrange in hierarchical tree structure
4. Select **Concentric** layout
   - **Expected**: Nodes arrange in concentric circles
5. Select **Force-Directed (COSE)** layout
   - **Expected**: Return to organic clustering layout

### Verification:
- [ ] All zoom controls work correctly
- [ ] Pan works by dragging
- [ ] Fit to screen centers graph
- [ ] Reset returns to initial state
- [ ] All 5 layouts work and animate smoothly
- [ ] Layout transitions are smooth (500ms animation)

---

## Test Scenario 5: Node Interaction

### Steps:
1. Click on a node (e.g., "ecommerce-api")
   - **Expected**:
     - Node gets red border (highlighted)
     - Console logs: `Node clicked: {id, name, ci_type, ...}`
2. Click on a different node
   - **Expected**:
     - Previous node loses red border
     - New node gets red border
     - Console logs new node data
3. Click on an edge (arrow)
   - **Expected**:
     - Console logs: `Edge clicked: {from, to, label, ...}`

### Verification:
- [ ] Nodes highlight on click (red border)
- [ ] Only one node highlighted at a time
- [ ] Console logs show correct node data
- [ ] Edge clicks log edge data
- [ ] No errors in console

---

## Test Scenario 6: Search Functionality

### Test 6.1: Basic Search
1. Type "ecommerce" in search box
   - **Expected**:
     - After 300ms debounce, autocomplete dropdown appears
     - Shows "ecommerce-api" result with purple color indicator
     - Shows "Application" as CI type
2. Clear search box
   - **Expected**: Dropdown disappears

### Test 6.2: Search and Select
1. Type "postgres" in search box
2. Click on "postgres-primary" in results
   - **Expected**:
     - Graph centers on postgres-primary node
     - Node is highlighted (amber glow)
     - Search box clears
     - Dropdown closes

### Test 6.3: No Results
1. Type "nonexistent" in search box
   - **Expected**: Dropdown shows "No results found"

### Verification:
- [ ] Search has 300ms debounce (doesn't search on every keystroke)
- [ ] Autocomplete shows matching results
- [ ] Results show CI type color indicators
- [ ] Clicking result centers and highlights node
- [ ] Clear button (X) works
- [ ] "No results found" shows for non-matching queries

---

## Test Scenario 7: CI Type Filtering

### Steps:
1. Select **"Application"** from CI type filter dropdown
   - **Expected**:
     - Graph refreshes
     - Shows only 3 application nodes (purple)
     - Shows relationships between applications
     - Controls show "3 nodes" and reduced edge count
2. Select **"Database"** from filter
   - **Expected**:
     - Shows only 2 database nodes (green)
     - Shows relationships involving databases
3. Select **"All CI Types"**
   - **Expected**:
     - Graph returns to showing all 8 nodes
     - All relationships visible again

### Verification:
- [ ] Filtering works correctly
- [ ] Node count updates
- [ ] Only selected type's nodes are visible
- [ ] Relationships are filtered appropriately
- [ ] "All CI Types" restores full graph

---

## Test Scenario 8: Error Handling

### Test 8.1: Backend Down
1. Stop the backend server (`Ctrl+C` in backend terminal)
2. Refresh the graph page
   - **Expected**:
     - Error alert appears: "Network error occurred" or similar
     - Graph shows empty state or previous data
     - No application crash

### Test 8.2: Invalid Filter
1. Restart backend
2. Manually edit URL to include invalid CI type: `?ci_type=InvalidType`
3. Navigate to that URL
   - **Expected**:
     - Graph shows 0 nodes (no matches)
     - No errors
     - UI remains functional

### Verification:
- [ ] Error messages display for network failures
- [ ] Application doesn't crash on errors
- [ ] Invalid filters don't cause errors
- [ ] User can recover from errors (refresh, etc.)

---

## Test Scenario 9: Neo4j Verification

### Steps:
1. Open Neo4j Browser: http://localhost:7474
2. Log in (User: neo4j, Password: dev12345)
3. Run Cypher query:
   ```cypher
   MATCH (n:CIAsset)-[r]->(m:CIAsset)
   RETURN n, r, m
   LIMIT 25
   ```
   - **Expected**:
     - Should see 8 nodes and 8 relationships
     - Nodes have properties: id, name, ci_type, attributes
     - Relationships have type (RUNS_ON, CONNECTS_TO, DEPENDS_ON)
4. Run node count query:
   ```cypher
   MATCH (n:CIAsset) RETURN count(n)
   ```
   - **Expected**: Returns 8
5. Run relationship count query:
   ```cypher
   MATCH ()-[r]->() RETURN count(r)
   ```
   - **Expected**: Returns 8

### Verification:
- [ ] Neo4j contains all 8 CI assets as nodes
- [ ] Neo4j contains all 8 relationships as edges
- [ ] Node properties match PostgreSQL data
- [ ] Relationship types are correctly named
- [ ] Graph structure matches expectations

---

## Test Scenario 10: Performance Testing

### Steps:
1. Note current node count (8 nodes)
2. Create additional test data:
   - Create 50 more Server assets
   - Create 50 more Application assets
   - Create relationships between them (100 total relationships)
3. Refresh graph page
   - **Expected**:
     - Graph loads within 2-3 seconds
     - All 108 nodes visible
     - Force-directed layout calculates positions
     - Graph remains interactive (zoom, pan work smoothly)
4. Test with 1000 node limit:
   - Verify graph doesn't attempt to load more than 1000 nodes
   - Controls show "1000 nodes" max

### Verification:
- [ ] Graph handles 100+ nodes smoothly
- [ ] Layout calculation completes in reasonable time
- [ ] Interactions remain responsive
- [ ] 1000 node limit is enforced
- [ ] No performance warnings in console

---

## Test Scenario 11: Responsive Design

### Steps:
1. Resize browser window to various sizes:
   - Full screen (1920x1080)
   - Medium (1280x720)
   - Small (1024x768)
2. Test on different screen sizes
   - **Expected**:
     - Controls adapt to screen size
     - Graph canvas resizes appropriately
     - Legend wraps to multiple rows on small screens
     - All UI elements remain accessible

### Verification:
- [ ] Layout is responsive
- [ ] No horizontal scrolling on small screens
- [ ] Controls remain visible and usable
- [ ] Graph adapts to container size

---

## Test Scenario 12: Legend Verification

### Steps:
1. Scroll down to the Legend card
2. Verify color indicators match graph nodes:
   - Server → Blue circle
   - Database → Green circle
   - Application → Purple circle
   - Network → Amber circle
   - Storage → Pink circle
   - Container → Cyan circle
   - Service → Indigo circle
   - Other → Gray circle

### Verification:
- [ ] Legend colors match graph node colors
- [ ] All 8 CI types are listed
- [ ] Color circles are visible and correctly colored
- [ ] Legend is easy to read

---

## Expected Test Results Summary

| Test Scenario | Expected Outcome | Status |
|---------------|------------------|--------|
| 1. Empty State | Displays correctly | ⬜ |
| 2. Create Test Data | All data created successfully | ⬜ |
| 3. Graph with Data | 8 nodes, 8 edges, correct colors | ⬜ |
| 4. Graph Controls | All controls work smoothly | ⬜ |
| 5. Node Interaction | Click, highlight, console logs work | ⬜ |
| 6. Search | Debounced search, autocomplete works | ⬜ |
| 7. Filtering | CI type filter works correctly | ⬜ |
| 8. Error Handling | Graceful error display | ⬜ |
| 9. Neo4j Verification | Data synced to Neo4j | ⬜ |
| 10. Performance | Handles 100+ nodes smoothly | ⬜ |
| 11. Responsive | Works on different screen sizes | ⬜ |
| 12. Legend | Colors match, all types listed | ⬜ |

---

## Common Issues and Solutions

### Issue 1: Graph Not Loading
**Symptoms**: Graph page shows loading spinner indefinitely
**Solutions**:
1. Check backend is running: `curl http://localhost:8080/health`
2. Check browser console for errors
3. Verify JWT token is valid (check localStorage `auth-storage`)
4. Check backend logs for errors

### Issue 2: Nodes Not Visible
**Symptoms**: Graph shows "0 nodes" but data exists
**Solutions**:
1. Verify CI assets exist in database
2. Check Neo4j has synced data (Neo4j Browser)
3. Try refreshing the page
4. Check CI type filter isn't filtering out all nodes

### Issue 3: Search Not Working
**Symptoms**: Search returns no results
**Solutions**:
1. Verify asset names in database
2. Check search is case-insensitive
3. Try searching with partial names
4. Check browser console for API errors

### Issue 4: Layout Issues
**Symptoms**: Nodes overlap or don't position correctly
**Solutions**:
1. Try different layout algorithms
2. Click "Fit to Screen"
3. Refresh the page
4. Check if too many nodes (>1000)

### Issue 5: Neo4j Sync Failures
**Symptoms**: Backend logs show Neo4j errors
**Solutions**:
1. Verify Neo4j is running: `docker-compose ps`
2. Check Neo4j credentials in backend `.env`
3. Restart Neo4j: `docker-compose restart neo4j`
4. Check Neo4j logs: `docker-compose logs neo4j`

---

## Debugging Checklist

- [ ] Backend is running on port 8080
- [ ] Frontend is running on port 3000
- [ ] PostgreSQL is running on port 5432
- [ ] Neo4j is running on port 7474 (browser) and 7687 (bolt)
- [ ] Redis is running on port 6379
- [ ] Browser console shows no errors
- [ ] Backend logs show no errors
- [ ] JWT token is valid in localStorage
- [ ] Database migrations have run
- [ ] CI types, assets, and relationships exist in PostgreSQL
- [ ] Neo4j contains synced data

---

## Next Steps After Testing

Once all tests pass:
1. ✅ Mark Phase 3 as complete
2. Document any issues found
3. Consider performance optimizations if needed
4. Plan Phase 4 (Import/Export & Audit System) or additional features
5. Deploy to staging/production environment

---

## Testing Completion Sign-Off

**Date**: _______________
**Tester**: _______________
**Phase 3 Status**: ⬜ All Tests Pass ⬜ Issues Found (see below)

**Issues Found**:
1. _____________________________________________
2. _____________________________________________
3. _____________________________________________

**Notes**:
_________________________________________________________________
_________________________________________________________________
_________________________________________________________________

---

**End of Testing Guide**
