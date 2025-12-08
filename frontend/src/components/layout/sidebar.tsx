import React from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/store/ui-store';
import { useAuth } from '@/hooks/use-auth';
import { Button } from '@/components/ui/button';
import {
  LayoutDashboard,
  Server,
  GitBranch,
  GitPullRequest,
  Package,
  Network,
  FileText,
  TrendingUp,
  Settings,
  LogOut,
  Menu,
  X,
  Users,
} from 'lucide-react';

const navigation = [
  {
    name: 'Dashboard',
    href: '/',
    icon: LayoutDashboard,
    roles: ['admin', 'user'],
  },
  {
    name: 'CI Management',
    href: '/ci-management',
    icon: Server,
    roles: ['admin', 'user'],
    children: [
      {
        name: 'Types',
        href: '/ci-management/types',
        icon: Package,
        roles: ['admin'],
      },
      {
        name: 'Lifecycles',
        href: '/ci-management/lifecycles',
        icon: GitBranch,
        roles: ['admin'],
      },
      {
        name: 'Relationships',
        href: '/ci-management/relationships',
        icon: GitPullRequest,
        roles: ['admin'],
      },
      {
        name: 'Assets',
        href: '/ci-management/assets',
        icon: Package,
        roles: ['admin', 'user'],
      },
      {
        name: 'Import',
        href: '/ci-management/import',
        icon: FileText,
        roles: ['admin'],
      },
    ],
  },
  {
    name: 'Graph',
    href: '/graph',
    icon: Network,
    roles: ['admin', 'user'],
  },
  {
    name: 'Audit Log',
    href: '/audit',
    icon: FileText,
    roles: ['admin'],
  },
  {
    name: 'Amortization',
    href: '/amortization',
    icon: TrendingUp,
    roles: ['admin', 'user'],
  },
  {
    name: 'Settings',
    href: '/settings',
    icon: Settings,
    roles: ['admin'],
  },
];

interface SidebarProps {
  className?: string;
}

export function Sidebar({ className }: SidebarProps) {
  const pathname = usePathname();
  const { sidebarOpen, setSidebarOpen } = useUIStore();
  const { user, logout } = useAuth();

  const filteredNavigation = React.useMemo(() => {
    if (!user) return [];

    return navigation.filter((item) => {
      if (!item.roles.includes(user.role)) return false;

      if (item.children) {
        item.children = item.children.filter((child) =>
          child.roles.includes(user.role)
        );

        return item.children.length > 0;
      }

      return true;
    });
  }, [user]);

  const isActive = (href: string) => {
    if (href === '/') {
      return pathname === href;
    }
    return pathname.startsWith(href);
  };

  const handleLogout = () => {
    logout();
  };

  return (
    <>
      {/* Mobile backdrop */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 z-40 bg-gray-600 bg-opacity-75 lg:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar */}
      <div
        className={cn(
          'fixed inset-y-0 left-0 z-50 w-64 bg-white shadow-lg transform transition-transform duration-300 ease-in-out lg:translate-x-0 lg:static lg:inset-0',
          sidebarOpen ? 'translate-x-0' : '-translate-x-full',
          className
        )}
      >
        <div className="flex flex-col h-full">
          {/* Header */}
          <div className="flex items-center justify-between h-16 px-4 border-b border-gray-200">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <img
                  className="h-8 w-auto"
                  src="/logo.svg"
                  alt="Crate"
                  onError={(e) => {
                    const target = e.target as HTMLImageElement;
                    target.style.display = 'none';
                    target.nextElementSibling?.classList.remove('hidden');
                  }}
                />
                <div className="hidden text-xl font-bold text-gray-900">
                  Crate
                </div>
              </div>
            </div>
            <Button
              variant="ghost"
              size="icon"
              className="lg:hidden"
              onClick={() => setSidebarOpen(false)}
            >
              <X className="h-6 w-6" />
            </Button>
          </div>

          {/* Navigation */}
          <nav className="flex-1 px-4 py-6 space-y-1 overflow-y-auto">
            {filteredNavigation.map((item) => (
              <div key={item.name}>
                <Link
                  href={item.href}
                  className={cn(
                    'flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors',
                    isActive(item.href)
                      ? 'bg-primary text-primary-foreground'
                      : 'text-gray-700 hover:bg-gray-100 hover:text-gray-900'
                  )}
                >
                  <item.icon
                    className={cn(
                      'flex-shrink-0 -ml-1 mr-3 h-5 w-5',
                      isActive(item.href) ? 'text-primary-foreground' : 'text-gray-400 group-hover:text-gray-500'
                    )}
                    aria-hidden="true"
                  />
                  {item.name}
                </Link>

                {/* Submenu */}
                {item.children && isActive(item.href) && (
                  <div className="ml-4 mt-1 space-y-1">
                    {item.children.map((child) => (
                      <Link
                        key={child.name}
                        href={child.href}
                        className={cn(
                          'flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors',
                          pathname === child.href
                            ? 'bg-primary text-primary-foreground'
                            : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
                        )}
                      >
                        <child.icon
                          className={cn(
                            'flex-shrink-0 -ml-1 mr-3 h-4 w-4',
                            pathname === child.href ? 'text-primary-foreground' : 'text-gray-400'
                          )}
                          aria-hidden="true"
                        />
                        {child.name}
                      </Link>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </nav>

          {/* User section */}
          <div className="border-t border-gray-200 p-4">
            <div className="flex items-center mb-4">
              <div className="flex-shrink-0">
                <div className="h-8 w-8 rounded-full bg-primary text-primary-foreground flex items-center justify-center">
                  {user?.name?.charAt(0).toUpperCase() || user?.email?.charAt(0).toUpperCase() || 'U'}
                </div>
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-700">{user?.name || 'Unknown User'}</p>
                <p className="text-xs text-gray-500 capitalize">{user?.role}</p>
              </div>
            </div>
            <div className="space-y-2">
              <Link href="/profile">
                <Button variant="ghost" className="w-full justify-start" size="sm">
                  <Users className="h-4 w-4 mr-2" />
                  Profile
                </Button>
              </Link>
              <Button
                variant="ghost"
                className="w-full justify-start text-red-600 hover:text-red-700 hover:bg-red-50"
                size="sm"
                onClick={handleLogout}
              >
                <LogOut className="h-4 w-4 mr-2" />
                Logout
              </Button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export function SidebarToggle() {
  const { sidebarOpen, setSidebarOpen } = useUIStore();

  return (
    <Button
      variant="ghost"
      size="icon"
      className="lg:hidden"
      onClick={() => setSidebarOpen(!sidebarOpen)}
    >
      {sidebarOpen ? (
        <X className="h-6 w-6" />
      ) : (
        <Menu className="h-6 w-6" />
      )}
    </Button>
  );
}