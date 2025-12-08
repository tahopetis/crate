# Crate - IT Asset Management Frontend

A comprehensive IT Asset Management Platform frontend built with Next.js 14, TypeScript, and modern web technologies.

## 🚀 Features

- **Modern Stack**: Next.js 14 with App Router, TypeScript, Tailwind CSS
- **UI Components**: Built with shadcn/ui component library
- **State Management**: Zustand for efficient state management
- **Authentication**: Secure login/register with token management
- **Responsive Design**: Mobile-first design with dark mode support
- **Type Safety**: Full TypeScript implementation
- **Form Handling**: React Hook Form with Zod validation

## 🛠️ Technology Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript 5.x
- **Styling**: Tailwind CSS + shadcn/ui
- **State Management**: Zustand
- **Forms**: React Hook Form + Zod
- **Icons**: Lucide React
- **Graph Visualization**: Cytoscape.js + react-cytoscapejs
- **Charts**: Recharts
- **Date Handling**: date-fns

## 📁 Project Structure

```
src/
├── app/                    # App Router pages
│   ├── auth/              # Authentication pages
│   ├── ci-management/     # CI management pages
│   ├── graph/             # Graph visualization
│   ├── audit/             # Audit log pages
│   ├── amortization/      # Amortization pages
│   └── layout.tsx         # Root layout
├── components/            # Reusable components
│   ├── ui/                # shadcn/ui components
│   ├── auth/              # Authentication components
│   ├── layout/            # Layout components
│   ├── dashboard/         # Dashboard components
│   ├── ci-management/     # CI management components
│   ├── graph/             # Graph components
│   ├── audit/             # Audit components
│   ├── amortization/      # Amortization components
│   └── common/            # Common utilities
├── lib/                   # Utility libraries
│   ├── api.ts             # API client
│   ├── auth.ts            # Authentication utilities
│   ├── types.ts           # TypeScript types
│   ├── validations.ts     # Form validation schemas
│   └── utils.ts           # Common utilities
├── hooks/                 # Custom React hooks
│   ├── use-auth.ts        # Authentication hook
│   ├── use-api.ts         # API request hook
│   ├── use-local-storage.ts
│   └── use-debounce.ts
├── store/                 # Zustand stores
│   ├── auth-store.ts      # Authentication state
│   ├── ci-store.ts        # CI data state
│   ├── ui-store.ts        # UI state
│   └── graph-store.ts     # Graph state
└── styles/                # Style files
    └── globals.css        # Global styles
```

## 🚀 Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Set up environment variables:
```bash
cp .env.local.example .env.local
```

Edit `.env.local` with your configuration:
```env
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_APP_NAME="Crate - IT Asset Management"
```

4. Start the development server:
```bash
npm run dev
```

5. Open [http://localhost:3000](http://localhost:3000) in your browser.

## 📚 Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server
- `npm run lint` - Run ESLint
- `npm run type-check` - Run TypeScript type checking

## 🔧 Configuration

### Environment Variables

Key environment variables:

- `NEXT_PUBLIC_API_URL` - Backend API URL
- `NEXT_PUBLIC_APP_NAME` - Application name
- `NEXT_PUBLIC_APP_VERSION` - Application version
- `NEXT_PUBLIC_DEFAULT_THEME` - Default theme (light/dark/system)

### API Configuration

The frontend is configured to work with a REST API. Update the `NEXT_PUBLIC_API_URL` environment variable to point to your backend server.

## 🎨 UI Components

This project uses shadcn/ui components, which provide:

- Accessible and customizable components
- Built with Radix UI primitives
- Tailwind CSS styling
- Dark mode support
- TypeScript support

## 🔐 Authentication

The authentication system includes:

- Login/Register forms with validation
- Token-based authentication
- Automatic token refresh
- Protected routes
- Role-based access control

## 📊 State Management

Zustand stores manage:

- **Authentication**: User state, tokens, loading states
- **CI Data**: Configuration Items, types, relationships
- **UI**: Theme preferences, loading states, notifications
- **Graph**: Graph visualization state

## 🧪 Development

### Code Style

This project uses:
- ESLint for code linting
- Prettier for code formatting
- TypeScript for type safety

### Component Development

- Use TypeScript interfaces for props
- Follow the established component structure
- Implement proper error boundaries
- Use the custom hooks for API calls

## 📱 Responsive Design

The application is built with a mobile-first approach:

- Responsive navigation with collapsible sidebar
- Touch-friendly interface elements
- Adaptive layouts for different screen sizes
- Optimized for both desktop and mobile devices

## 🌙 Dark Mode

Built-in dark mode support:

- System preference detection
- Manual theme switching
- Persistent theme preference
- Component-level theme awareness

## 📈 Features Implemented

### ✅ Completed
- Project setup with Next.js 14 + TypeScript
- Authentication system (login/register)
- Layout components (header, sidebar, navigation)
- Dashboard with stats and activity
- Basic routing structure
- shadcn/ui components integration
- State management with Zustand
- API client configuration
- Form validation with Zod

### 🚧 In Progress
- CI Types management
- CI Assets management
- Graph visualization
- Audit log viewer
- Amortization calculations
- Import/Export functionality

### 📋 Planned
- Advanced search and filtering
- Real-time notifications
- Advanced graph layouts
- Custom reports
- Integration with external systems
- Mobile app version

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For support, please:
- Check the documentation
- Create an issue in the repository
- Contact the development team

## 🔗 Related Projects

- [Crate Backend](../backend) - Rust API service
- [Crate Database](../database) - Database schemas and migrations
