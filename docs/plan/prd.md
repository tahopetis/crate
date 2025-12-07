---

# **📘 Product Requirements Document (PRD)**

## **Crate - IT Asset Management Platform (MVP-1)**

**Version:** 1.0
**Audience:** Engineering, Architecture, UX, DevOps
**Tech Stack:** Next.js + shadcn, Rust, PostgreSQL (JSONB), Neo4j

---

# **1. Product Overview**

Crate, The IT Asset Management Platform is a next-generation, graph-enabled asset repository designed to evolve into a full Enterprise Architecture (EA) tool.
MVP-1 focuses on:

* Flexible CI Types
* Dynamic attributes (JSONB)
* Configurable Lifecycle and Relationship types
* Asset CRUD and import/export
* Asset relationship graph visualization (Neo4j)
* Auditability across all changes
* Foundational dashboard metrics

This system must be **metadata-driven**, **schema-flexible**, and **graph-aware**, forming the backbone for future EA capabilities.

---

# **2. Goals & Non-Goals**

## **2.1 Goals (MVP-1)**

1. Provide a **flexible meta-model**:

   * CI Types
   * Lifecycle definitions
   * Relationship types
2. Manage assets with dynamic attributes stored in JSONB.
3. Manage asset relationships stored in Neo4j.
4. Provide a basic Graph view with filtering & search.
5. Track all modifications via Audit Log.
6. Provide essential dashboard KPIs.
7. Allow traditional IT onboarding via CSV Import/Export.
8. Implement daily amortization calculation.


---

# **3. User Roles**

| Role   | Description                   | Permissions    |
| ------ | ----------------------------- | -------------- |
| Admin  | Configures system meta-model  | Full access    |
| Editor | Manages CIs and relationships | CRUD on assets |
| Viewer | Read-only user                | View + export  |

MVP-1 may introduce a simplified 2-role system (Admin & Viewer) if needed.

---

# **4. High-Level System Architecture**

## **4.1 Components**

* **Frontend**: Next.js, shadcn
* **Backend**: Rust
* **Database**: PostgreSQL
* **Graph Database**: Neo4j
* **Background Jobs**: Rust scheduled tasks
* **Audit Engine**: PostgreSQL table + JSONB diffs

## **4.2 Data Division**

* **Postgres** → CI metadata, CI attributes, types, lifecycles
* **Neo4j** → CI relationships
* **Rust** → API, schema validation, amortization jobs
* **Next.js** → views, filters, interactions

---

# **5. Core Features**

---

# **5.1 Dashboard**

## **Description**

A high-level overview of the asset inventory and key operational metrics.

## **Requirements**

* Display total CI count
* CI count by CI Type
* CI count by lifecycle
* Top 10 CIs with highest relationship counts
* Daily amortization value graph (last 30 days)

## **Acceptance Criteria**

* Dashboard loads within <2 seconds
* Data reflects yesterday’s amortization job results
* Clicking a CI Type filters CI list

---

# **5.2 CI Management**

This section includes Type, Lifecycle, Relationship, and Assets.

---

## **5.2.1 CI Types**

### **Description**

Allows admins to define the schema (fields) and metadata structure for each CI type.

### **Data Model**

`ci_types` table:

* id (UUID)
* name (string, unique)
* description (text)
* schema_json (JSONB) → includes:

  * field name
  * field type (text, number, boolean, date, enum)
  * required flag
  * default value (optional)

### **Capabilities**

* Create CI Type
* Edit CI Type (versioning optional for MVP)
* Delete CI Type (only if no linked assets)
* Validate CI assets against CI Type schema

### **Acceptance Criteria**

* Cannot delete a type with existing assets
* Schema validation enforced during asset insert/update

---

## **5.2.2 Lifecycle**

### **Description**

Define lifecycle states applicable across CIs.

### **Data Model**

`ci_lifecycles`:

* id
* name
* description
* order (optional)

### **Capabilities**

* Add/Edit/Delete lifecycle states
* Assign to CIs
* Restrict deletion if lifecycle is in use

### **Acceptance Criteria**

* User cannot delete lifecycle with allocated CIs

---

## **5.2.3 Relationship Types**

### **Description**

Defines allowable relationship types stored in Neo4j.

### **Data Model**

`relationship_types`:

* id
* name
* description

### **Capabilities**

* Add/Edit/Delete relationship types
* Used when connecting CIs in Assets section

### **Acceptance Criteria**

* Relationship type is validated before creating graph edge

---

## **5.2.4 Assets**

### **Description**

Assets are configuration items (CIs) with dynamic attributes.

### **Data Model**

`cis`:

* id
* ci_type_id
* name
* attributes_json (JSONB)
* lifecycle_id
* created_at
* updated_at

### **Features**

* Add/Edit/Delete assets
* JSONB dynamic attribute UI generated based on CI Type schema
* Import via CSV
* Export as CSV
* Link/unlink relationships (calls Neo4j)

### **Import Specification**

CSV headers:

* `name`
* `ci_type`
* `lifecycle`
* Dynamic attribute fields
* `relationships` (optional JSON inline array)

### **Relationship Management**

In edit asset:

* Add relationship
* Remove relationship
* Relationship types determined by Relationship Types table

Neo4j:

```
MATCH (a:CI {id: $id1}), (b:CI {id: $id2})
CREATE (a)-[:DEPENDS_ON]->(b)
```

### **Acceptance Criteria**

* JSON schema validation must pass
* Import must support >5000 rows
* Export must reflect full attributes

---

# **5.3 Graph**

### **Description**

Visualizes CI relationships using Neo4j.

### **Features (MVP-1)**

#### **1. Filter by CI Type**

* Dropdown
* Filters nodes by type
* Non-matching nodes are hidden or dimmed

#### **2. Search CI Name (autocomplete)**

* Suggest CIs based on partial name
* When clicked:

  * Graph centers on the node
  * Highlights node
  * Expands immediate neighbors

#### **3. Graph Navigation**

* Pan
* Zoom
* Drag nodes

#### **4. Node Detail Panel**

Displays:

* CI name
* Type
* Lifecycle
* Attributes
* List of relationships

### **Frontend Library**

* Cytoscape.js or D3.js
  (Cytoscape recommended for fast implementation)

### **Acceptance Criteria**

* Graph loads <3 seconds for 1000 nodes
* Filter updates in <800ms
* Search results must return within <200ms

---

# **5.4 Audit Log**

### **Description**

Tracks all changes across:

* CI Types
* Lifecycle
* Relationship Types
* Assets (CI)
* Relationship changes
* Imports

### **Data Model**

`audit_log`:

* id
* user_id
* timestamp
* action (enum)
* entity_type
* entity_id
* before_json
* after_json

### **Captured Events**

* create / update / delete for all entities
* CSV import events (one log entry per asset or per batch)
* Graph relationship changes

### **UI**

* Paginated table
* Filters:

  * date
  * user
  * entity type
  * action

### **Acceptance Criteria**

* Must store JSON diff or full before/after
* Must handle high write volume during import

---

# **5.5 Amortization Engine**

### **Description**

Daily job to calculate:

* Asset value
* Accumulated depreciation
* Remaining value

### **Formula**

Straight-line depreciation:

```
daily_depreciation = purchase_cost / useful_life_days
accumulated = daily_depreciation * days_in_use
value = purchase_cost - accumulated
```

### **Requirements**

* Runs every midnight
* Stores results in a new table:

`ci_valuation_daily`:

* ci_id
* date
* value
* accumulated

### **Acceptance Criteria**

* Must process at least 10k assets within < 30 seconds
* Dashboard reads precomputed values

---

# **6. API Requirements**

### **Architecture**

* REST (optional GraphQL later)
* Token-based auth (simple for MVP)

### **Key Endpoints**

* `/ci-types/*`
* `/lifecycles/*`
* `/relationship-types/*`
* `/cis/*`
* `/cis/import`
* `/ci/search?query=`
* `/graph?type=&id=`
* `/audit/*`

---

# **7. Non-Functional Requirements (NFR)**

## **Performance**

* API responses < 300ms
* Graph rendering < 3s with moderate dataset

## **Scalability**

* Must handle 100k CIs
* Must support 500k relationships

## **Security**

* RBAC
* Basic rate limiting

## **Reliability**

* Daily amortization job must retry upon failure

## **Auditability**

* All changes tracked

---

# **8. User Interface (UI) Overview**

## **8.1 Left Menu**

* Dashboard
* CI Management

  * Type
  * Lifecycle
  * Relationship
  * Assets
* Graph
* Audit Log

## **8.2 UI Style**

* shadcn components
* clean, minimal enterprise look (banking standard)

---

# **9. Future Expansion Roadmap (v2+)**

Not part of MVP, but the system is designed for:

* Enterprise Architecture tool
* Business capabilities
* Process mapping
* Integration architecture
* Roadmapping & timeline views
* Portfolio cost modeling
* Compliance scoring
* Auto-diagram generation
* Multi-tenancy
* SSO + SCIM

---

# **10. Acceptance Criteria Summary**

✔ CI Type CRUD works with JSON schema validation
✔ Lifecycle & Relationship CRUD functional
✔ Asset CRUD + JSONB dynamic fields
✔ Import/export fully functional
✔ Neo4j graph displays relationships correctly
✔ Graph filters (type + search) functional
✔ Audit log shows all changes
✔ Amortization job runs daily and updates dashboard
✔ System stable with at least 10k CIs

---

# **11. Final Deliverables (Engineering)**

* Backend Rust API
* Postgres schema
* Neo4j schema
* Next.js pages & components
* Background task scheduler
* Audit logging system
* Import/export utility
* Deployment manifests (Docker/Kubernetes (optional))

---

