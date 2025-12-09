'use client';

import React from 'react';
import { usePathname } from 'next/navigation';
import { Sidebar } from './sidebar';
import { Header } from './header';
import { Toaster } from '@/components/ui/toaster';
import { useAuth } from '@/hooks/use-auth';

interface MainLayoutProps {
  children: React.ReactNode;
}

export function MainLayout({ children }: MainLayoutProps) {
  const pathname = usePathname();
  const { isAuthenticated } = useAuth();

  // Don't show sidebar and header on auth pages
  const isAuthPage = pathname?.startsWith('/auth');
  const showFullLayout = isAuthenticated && !isAuthPage;

  if (!showFullLayout) {
    return (
      <>
        {children}
        <Toaster />
      </>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="flex">
        <Sidebar />
        <div className="flex-1 lg:ml-64">
          <Header />
          <main className="flex-1">
            <div className="py-6">
              <div className="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
                {children}
              </div>
            </div>
          </main>
        </div>
      </div>
      <Toaster />
    </div>
  );
}

export default MainLayout;