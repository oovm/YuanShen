# Presentation Layer (Application Layer)

The presentation layer is the interface where users directly interact with the system, including various applications and frontend interfaces. This layer resides at the top of the three-layer architecture and provides services by calling the core modules of the implementation layer.

## Overall Architecture

The presentation layer consists of two main components:
- **Frontend Applications** - Located in the `frontends/` directory, providing user interfaces
- **Backend Applications** - Located in the `backends/` directory, providing server-side support

### Presentation Layer Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend Applications (frontends/)         │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (Main App)  │  │  (Empire)    │  │  (Planet)    │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-waifu    │  │  client-h5    │  │client-desktop│    │
│  │  (Character) │  │  (H5 Client)  │  │(Desktop Client)│  │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │client-mobile │  │ client-shared │                        │
│  │  (Mobile)    │  │  (Shared Lib)│                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↓ HTTP/WebSocket
┌─────────────────────────────────────────────────────────────┐
│                    Backend Applications (backends/)           │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (Main App)  │  │  (Empire)    │  │  (Planet)    │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐                                            │
│  │  ai-waifu    │                                            │
│  │  (Character) │                                            │
│  └──────────────┘                                            │
└─────────────────────────────────────────────────────────────┘
                            ↓ Call
┌─────────────────────────────────────────────────────────────┐
│              Implementation Layer (augur-* Core Modules)      │
└─────────────────────────────────────────────────────────────┘
```

## Frontend Applications Detailed Description

### 1. ai-company (Main Application)

**Directory Location**: `frontends/ai-company/`

**Tech Stack**:
- Vue 3 + TypeScript
- Vite build tool
- Element Plus UI component library
- Vue Router routing
- Pinia state management
- UnoCSS atomic CSS
- Fluent Vue internationalization

**Port**: Development server default port configured via Vite

**Responsibilities**:
- Marketing homepage display
- User login/registration
- Company management
- Employee management
- Team management
- Project management
- Forum features
- Administrator features

**Main Views**:
- `Home.vue` - Homepage
- `Login.vue` - Login page
- `Register.vue` - Registration page
- `Company.vue` - Company list
- `CompanyDetail.vue` - Company detail
- `Employees.vue` - Employee list
- `EmployeeDetail.vue` - Employee detail
- `Team.vue` - Team list
- `TeamDetail.vue` - Team detail
- `Project.vue` - Project list
- `ProjectDetail.vue` - Project detail
- `Forum.vue` - Forum
- `ForumDetail.vue` - Forum detail
- `Dashboard.vue` - Dashboard
- `Download.vue` - Download page
- `admin/AdminDashboard.vue` - Admin dashboard
- `admin/Users.vue` - User management

### 2. ai-empire (Empire Module)

**Directory Location**: `frontends/ai-empire/`

**Tech Stack**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI component library
- Vue Router routing
- Pinia state management
- UnoCSS atomic CSS
- Fluent Vue internationalization
- TipTap rich text editor

**Responsibilities**:
- Empire management interface
- AI Advisor management
- AI Consultant management
- Legion management
- Great Work management
- User authentication
- Dashboard display

**Main Views**:
- `Home.vue` - Homepage
- `Login.vue` - Login page
- `Register.vue` - Registration page
- `Empire.vue` - Empire list
- `EmpireDetail.vue` - Empire detail
- `Advisors.vue` - Advisor list
- `AdvisorDetail.vue` - Advisor detail
- `Consultants.vue` - Consultant list
- `ConsultantDetail.vue` - Consultant detail
- `Legion.vue` - Legion list
- `LegionDetail.vue` - Legion detail
- `GreatWork.vue` - Great Work list
- `GreatWorkDetail.vue` - Great Work detail

**Featured Functions**:
- AI employee management: Treat AI agents as enterprise employees
- Characterized AI assistants: Customer service, finance, marketing, administrative assistant, legal, etc.
- Knowledge management system
- Rich text editing capabilities

### 3. ai-planet (Planet Module)

**Directory Location**: `frontends/ai-planet/`

**Tech Stack**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI component library
- Vue Router routing
- Pinia state management
- UnoCSS atomic CSS
- Fluent Vue internationalization
- TipTap rich text editor

**Responsibilities**:
- Planet-related feature interfaces
- User authentication

**Main Views**:
- `Home.vue` - Homepage
- `Auth.vue` - Authentication page

### 4. ai-waifu (Character Module)

**Directory Location**: `frontends/ai-waifu/`

**Tech Stack**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI component library
- Vue Router routing
- Pinia state management
- UnoCSS atomic CSS
- Fluent Vue internationalization
- TipTap rich text editor

**Responsibilities**:
- AI character chat interface
- Character interaction

**Main Views**:
- `Home.vue` - Homepage
- `Chat.vue` - Chat interface

### 5. client-h5 (H5 Client)

**Directory Location**: `frontends/client-h5/`

**Tech Stack**:
- Vue 3 + TypeScript
- Vite build tool
- Element Plus UI component library
- Vue Router routing
- Pinia state management
- UnoCSS atomic CSS

**Responsibilities**:
- Mobile H5 application
- Workbench view
- Augur collaboration mode
- Enterprise WeChat-style mode
- Application management
- Message center
- Contact management
- Calendar
- Meetings
- Email
- Knowledge base
- Memory management
- Project management

**Featured Functions**:
- Work mode switching: Augur collaboration mode / Enterprise WeChat-style mode
- Multi-application integration: Attendance, CRM, contracts, finance, HR, performance, recruitment, Wiki, workflows, etc.
- Multiple layout modes

### 6. client-desktop (Desktop Client)

**Directory Location**: `frontends/client-desktop/`

**Tech Stack**:
- Tauri (Rust + WebView)
- Cross-platform desktop application

**Responsibilities**:
- Desktop application
- Provide native desktop experience

### 7. client-mobile (Mobile Client)

**Directory Location**: `frontends/client-mobile/`

**Tech Stack**:
- Tauri (Rust + WebView)
- Mobile application

**Responsibilities**:
- Mobile application

### 8. client-shared (Shared Library)

**Directory Location**: `frontends/client-shared/`

**Tech Stack**:
- Vue 3 + TypeScript

**Responsibilities**:
- Shared component library
- Shared services (API calls)
- Shared state management (Pinia stores)
- Shared type definitions
- Shared utility functions
- Shared internationalization resources
- Shared plugins

**Contents**:
- `components/` - Shared Vue components
- `services/` - API services and mock data
- `stores/` - Pinia state management
- `types/` - TypeScript type definitions
- `utils/` - Utility functions
- `locales/` - Internationalization resources
- `plugins/` - Vue plugins

## Backend Applications Detailed Description

### 1. ai-company (Main Application Backend)

**Directory Location**: `backends/ai-company/`

**Tech Stack**:
- Rust + Tokio async runtime
- Axum web framework
- Serde serialization
- Tracing logging
- rust-embed static resource embedding

**Port**: 4002

**Responsibilities**:
- Provide HTTP services
- Embed and serve frontend static resources
- Routing and static file serving
- Provide server-side support for main application frontend

**Main Features**:
- Static file serving: Embed frontend build artifacts from `frontends/ai-company/dist` directory
- SPA routing support: All unmatched paths return index.html
- Static resource MIME type auto-detection

### 2. ai-empire (Empire Module Backend)

**Directory Location**: `backends/ai-empire/`

**Tech Stack**:
- Rust + Tokio async runtime
- Axum web framework
- Serde serialization
- Tracing logging
- rust-embed static resource embedding

**Responsibilities**:
- Provide HTTP services
- Embed and serve Empire module frontend static resources
- Provide server-side support for Empire module frontend

### 3. ai-planet (Planet Module Backend)

**Directory Location**: `backends/ai-planet/`

**Tech Stack**:
- Rust + Tokio async runtime
- Axum web framework
- Serde serialization
- Tracing logging
- rust-embed static resource embedding

**Responsibilities**:
- Provide HTTP services
- Embed and serve Planet module frontend static resources
- Provide server-side support for Planet module frontend

### 4. ai-waifu (Character Module Backend)

**Directory Location**: `backends/ai-waifu/`

**Tech Stack**:
- Rust + Tokio async runtime
- Axum web framework
- Serde serialization
- Tracing logging
- rust-embed static resource embedding

**Responsibilities**:
- Provide HTTP services
- Embed and serve Character module frontend static resources
- Provide server-side support for Character module frontend

## Application Relationships

### Frontend Application Relationships

```
client-shared (Shared Library)
    ↑ Depends on
    ├─→ ai-company (Main App)
    ├─→ ai-empire (Empire)
    ├─→ ai-planet (Planet)
    ├─→ ai-waifu (Character)
    └─→ client-h5 (H5 Client)

client-h5 (H5 Client)
    ↑ Reference/Inspiration
    ├─→ client-desktop (Desktop Client)
    └─→ client-mobile (Mobile Client)
```

### Frontend-Backend Pairing Relationships

| Frontend Application | Backend Application | Description |
|----------------------|---------------------|-------------|
| `ai-company` | `ai-company` | Main application frontend-backend pairing |
| `ai-empire` | `ai-empire` | Empire module frontend-backend pairing |
| `ai-planet` | `ai-planet` | Planet module frontend-backend pairing |
| `ai-waifu` | `ai-waifu` | Character module frontend-backend pairing |
| `client-h5` | (TBD) | H5 client backend |
| `client-desktop` | (TBD) | Desktop client backend |
| `client-mobile` | (TBD) | Mobile client backend |

### Relationship with Implementation Layer

All backend applications ultimately call the `augur-*` core modules of the implementation layer to execute business logic:

```
Presentation Layer (Backend Applications)
    ↓ Calls
Implementation Layer (augur-* Core Modules)
    ↓ Uses
Protocol Layer (Skynet Protocol)
```

## Deployment Architecture

### Development Environment

- Frontend applications run independently via Vite dev server
- Backend applications run independently via Cargo
- Frontend and backend communicate via API calls

### Production Environment

- Frontend build artifacts embedded into corresponding backend applications (via rust-embed)
- Each backend application runs as an independent service, providing complete frontend-backend functionality
- Each service listens on different ports

## Technology Choices Explanation

### Frontend Technology Choices

- **Vue 3**: Progressive JavaScript framework providing excellent development experience and performance
- **TypeScript**: Provides type safety, improving code maintainability
- **Vite**: Next-generation frontend build tool providing extremely fast development experience
- **Element Plus**: Vue 3-based component library providing rich UI components
- **Pinia**: Vue 3 official recommended state management library
- **UnoCSS**: Atomic CSS engine providing flexible styling solutions
- **Fluent Vue**: Internationalization solution supporting multiple languages

### Backend Technology Choices

- **Rust**: High-performance, memory-safe systems programming language
- **Tokio**: Rust async runtime providing high-performance I/O handling
- **Axum**: Ergonomic and modular web framework providing good development experience
- **rust-embed**: Embeds frontend build artifacts into Rust binary files, simplifying deployment

## Development Guide

### Frontend Development

```bash
# Enter frontend application directory
cd frontends/ai-company

# Install dependencies
npm install

# Start dev server
npm run dev

# Build production version
npm run build
```

### Backend Development

```bash
# Enter backend application directory
cd backends/ai-company

# Run dev server
cargo run

# Build production version
cargo build --release
```

### Complete Development Workflow

1. First build the frontend application
2. Then run the backend application (which will embed the frontend build artifacts)

```bash
# Build frontend
cd frontends/ai-company
npm run build

# Run backend
cd ../../backends/ai-company
cargo run
```
