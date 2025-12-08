'use client';

import React from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/hooks/use-auth';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import {
  Server,
  Network,
  FileText,
  TrendingUp,
  Plus,
  Activity,
  AlertTriangle,
  DollarSign,
} from 'lucide-react';
import Link from 'next/link';

export default function Dashboard() {
  const { user, isAuthenticated } = useAuth();
  const router = useRouter();

  React.useEffect(() => {
    if (!isAuthenticated) {
      router.push('/auth/login');
    }
  }, [isAuthenticated, router]);

  if (!isAuthenticated) {
    return null;
  }

  const stats = [
    {
      title: 'Total Assets',
      value: '247',
      change: '+12%',
      icon: Server,
      color: 'text-blue-600',
      bgColor: 'bg-blue-100',
    },
    {
      title: 'Active Relationships',
      value: '89',
      change: '+5%',
      icon: Network,
      color: 'text-green-600',
      bgColor: 'bg-green-100',
    },
    {
      title: 'Audit Logs',
      value: '1,429',
      change: '+18%',
      icon: FileText,
      color: 'text-purple-600',
      bgColor: 'bg-purple-100',
    },
    {
      title: 'Total Value',
      value: '$2.4M',
      change: '+3%',
      icon: DollarSign,
      color: 'text-yellow-600',
      bgColor: 'bg-yellow-100',
    },
  ];

  const quickActions = [
    {
      title: 'Add New Asset',
      description: 'Create a new CI asset',
      icon: Plus,
      href: '/ci-management/assets/new',
      color: 'bg-blue-500',
    },
    {
      title: 'View Graph',
      description: 'Explore asset relationships',
      icon: Network,
      href: '/graph',
      color: 'bg-green-500',
    },
    {
      title: 'Import Data',
      description: 'Import assets from CSV',
      icon: FileText,
      href: '/ci-management/import',
      color: 'bg-purple-500',
    },
    {
      title: 'View Reports',
      description: 'Generate amortization reports',
      icon: TrendingUp,
      href: '/amortization',
      color: 'bg-yellow-500',
    },
  ];

  const recentActivity = [
    {
      id: 1,
      action: 'Created new asset',
      entity: 'Server-Web-01',
      user: 'John Doe',
      timestamp: '2 minutes ago',
      type: 'create',
    },
    {
      id: 2,
      action: 'Updated relationship',
      entity: 'Server-Web-01 ↔ DB-Prod-01',
      user: 'Jane Smith',
      timestamp: '15 minutes ago',
      type: 'update',
    },
    {
      id: 3,
      action: 'Deleted asset',
      entity: 'Workstation-Old-05',
      user: 'Bob Johnson',
      timestamp: '1 hour ago',
      type: 'delete',
    },
    {
      id: 4,
      action: 'Created lifecycle',
      entity: 'Development Lifecycle',
      user: 'Alice Brown',
      timestamp: '2 hours ago',
      type: 'create',
    },
  ];

  const getActivityIcon = (type: string) => {
    switch (type) {
      case 'create':
        return <Plus className="h-4 w-4 text-green-500" />;
      case 'update':
        return <Activity className="h-4 w-4 text-blue-500" />;
      case 'delete':
        return <AlertTriangle className="h-4 w-4 text-red-500" />;
      default:
        return <FileText className="h-4 w-4 text-gray-500" />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Welcome Header */}
      <div>
        <h1 className="text-2xl font-semibold text-gray-900">
          Welcome back, {user?.name || 'User'}!
        </h1>
        <p className="mt-1 text-sm text-gray-600">
          Here&apos;s what&apos;s happening with your IT assets today.
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <Card key={stat.title}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">{stat.title}</CardTitle>
              <div className={`p-2 rounded-full ${stat.bgColor}`}>
                <stat.icon className={`h-4 w-4 ${stat.color}`} />
              </div>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stat.value}</div>
              <p className="text-xs text-muted-foreground">
                <span className="text-green-600">{stat.change}</span> from last month
              </p>
            </CardContent>
          </Card>
        ))}
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        {/* Quick Actions */}
        <Card>
          <CardHeader>
            <CardTitle>Quick Actions</CardTitle>
            <CardDescription>
              Common tasks you might want to perform
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {quickActions.map((action) => (
              <Link key={action.title} href={action.href}>
                <div className="flex items-center space-x-3 p-3 rounded-lg border hover:bg-gray-50 transition-colors cursor-pointer">
                  <div className={`p-2 rounded-full ${action.color}`}>
                    <action.icon className="h-4 w-4 text-white" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-gray-900 truncate">
                      {action.title}
                    </p>
                    <p className="text-sm text-gray-500 truncate">
                      {action.description}
                    </p>
                  </div>
                </div>
              </Link>
            ))}
          </CardContent>
        </Card>

        {/* Recent Activity */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <div className="flex items-center justify-between">
              <div>
                <CardTitle>Recent Activity</CardTitle>
                <CardDescription>
                  Latest changes to your IT assets
                </CardDescription>
              </div>
              <Link href="/audit">
                <Button variant="outline" size="sm">
                  View All
                </Button>
              </Link>
            </div>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {recentActivity.map((activity) => (
                <div
                  key={activity.id}
                  className="flex items-start space-x-3 p-3 rounded-lg border"
                >
                  <div className="mt-0.5">
                    {getActivityIcon(activity.type)}
                  </div>
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-gray-900">
                      {activity.action}
                    </p>
                    <p className="text-sm text-gray-500 truncate">
                      {activity.entity}
                    </p>
                    <div className="flex items-center space-x-2 mt-1">
                      <span className="text-xs text-gray-400">
                        {activity.user}
                      </span>
                      <span className="text-xs text-gray-400">•</span>
                      <span className="text-xs text-gray-400">
                        {activity.timestamp}
                      </span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Getting Started */}
      <Card>
        <CardHeader>
          <CardTitle>Getting Started</CardTitle>
          <CardDescription>
            New to Crate? Here are some resources to help you get started.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <div className="p-4 border rounded-lg">
              <h4 className="font-medium mb-2">1. Define CI Types</h4>
              <p className="text-sm text-gray-600 mb-3">
                Create custom CI types to classify your assets
              </p>
              <Link href="/ci-management/types">
                <Button variant="outline" size="sm">
                  Manage Types
                </Button>
              </Link>
            </div>
            <div className="p-4 border rounded-lg">
              <h4 className="font-medium mb-2">2. Import Assets</h4>
              <p className="text-sm text-gray-600 mb-3">
                Import your existing assets from CSV or Excel files
              </p>
              <Link href="/ci-management/import">
                <Button variant="outline" size="sm">
                  Import Data
                </Button>
              </Link>
            </div>
            <div className="p-4 border rounded-lg">
              <h4 className="font-medium mb-2">3. Create Relationships</h4>
              <p className="text-sm text-gray-600 mb-3">
                Define relationships between your assets
              </p>
              <Link href="/graph">
                <Button variant="outline" size="sm">
                  View Graph
                </Button>
              </Link>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
