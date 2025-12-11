'use client';

import React, { useState, useEffect, useCallback, useRef } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { AlertCircle, RefreshCw } from 'lucide-react';
import { Button } from '@/components/ui/button';
import CytoscapeGraph, { GraphData, GraphNode } from '@/components/graph/cytoscape-graph';
import GraphControls from '@/components/graph/graph-controls';
import GraphSearch from '@/components/graph/graph-search';
import CITypeFilter from '@/components/graph/ci-type-filter';
import { apiClient, apiEndpoints } from '@/lib/api';
import { ApiResponse } from '@/lib/types';

interface CIType {
  id: string;
  name: string;
}

export default function GraphPage() {
  const [graphData, setGraphData] = useState<GraphData>({ nodes: [], edges: [] });
  const [ciTypes, setCiTypes] = useState<CIType[]>([]);
  const [selectedCiType, setSelectedCiType] = useState<string | null>(null);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [layout, setLayout] = useState<'cose' | 'circle' | 'grid' | 'breadthfirst' | 'concentric'>('cose');
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const cyRef = useRef<any>(null);

  // Fetch CI Types for filter
  const fetchCITypes = useCallback(async () => {
    try {
      const response = await apiClient.get<ApiResponse<CIType[]>>(apiEndpoints.ci.types);
      if (response.success && response.data) {
        setCiTypes(response.data);
      }
    } catch (err) {
      console.error('Failed to fetch CI types:', err);
    }
  }, []);

  // Fetch graph data
  const fetchGraphData = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const params: any = { limit: 1000 };
      if (selectedCiType) {
        params.ci_type = selectedCiType;
      }

      const response = await apiClient.get<ApiResponse<GraphData>>(
        apiEndpoints.graph.data,
        params
      );

      if (response.success && response.data) {
        setGraphData(response.data);
      } else {
        setError(response.message || 'Failed to load graph data');
      }
    } catch (err: any) {
      setError(err.message || 'An error occurred while loading the graph');
      console.error('Graph data fetch error:', err);
    } finally {
      setIsLoading(false);
    }
  }, [selectedCiType]);

  // Search nodes
  const handleSearch = useCallback(async (query: string): Promise<GraphNode[]> => {
    try {
      const response = await apiClient.get<ApiResponse<GraphNode[]>>(
        apiEndpoints.graph.search,
        { q: query, limit: 10 }
      );

      if (response.success && response.data) {
        return response.data;
      }
      return [];
    } catch (err) {
      console.error('Search error:', err);
      return [];
    }
  }, []);

  // Graph control handlers
  const handleZoomIn = () => {
    if (typeof window !== 'undefined' && (window as any).cy) {
      (window as any).cy.zoom((window as any).cy.zoom() * 1.2);
    }
  };

  const handleZoomOut = () => {
    if (typeof window !== 'undefined' && (window as any).cy) {
      (window as any).cy.zoom((window as any).cy.zoom() * 0.8);
    }
  };

  const handleFit = () => {
    if (typeof window !== 'undefined' && (window as any).cy) {
      (window as any).cy.fit(50);
    }
  };

  const handleReset = () => {
    if (typeof window !== 'undefined' && (window as any).cy) {
      (window as any).cy.reset();
    }
    setSelectedNodeId(null);
  };

  const handleNodeClick = (node: GraphNode) => {
    console.log('Node clicked:', node);
    setSelectedNodeId(node.id);
  };

  const handleEdgeClick = (edge: any) => {
    console.log('Edge clicked:', edge);
  };

  const handleSelectNode = (nodeId: string) => {
    setSelectedNodeId(nodeId);
  };

  // Initial load
  useEffect(() => {
    fetchCITypes();
    fetchGraphData();
  }, [fetchCITypes, fetchGraphData]);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold text-gray-900">Asset Relationship Graph</h1>
          <p className="mt-1 text-sm text-gray-600">
            Visualize relationships between your configuration items
          </p>
        </div>
        <Button
          onClick={fetchGraphData}
          disabled={isLoading}
          variant="outline"
        >
          <RefreshCw className={`w-4 h-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
          Refresh
        </Button>
      </div>

      {/* Error Alert */}
      {error && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Filters and Search */}
      <div className="flex items-center justify-between gap-4">
        <CITypeFilter
          ciTypes={ciTypes}
          selectedType={selectedCiType}
          onTypeChange={setSelectedCiType}
          isLoading={isLoading}
        />
        <GraphSearch
          onSearch={handleSearch}
          onSelectNode={handleSelectNode}
          placeholder="Search assets..."
        />
      </div>

      {/* Graph Controls */}
      <GraphControls
        onZoomIn={handleZoomIn}
        onZoomOut={handleZoomOut}
        onFit={handleFit}
        onReset={handleReset}
        layout={layout}
        onLayoutChange={(value) => setLayout(value as any)}
        nodeCount={graphData.nodes.length}
        edgeCount={graphData.edges.length}
      />

      {/* Graph Visualization */}
      <Card>
        <CardHeader>
          <CardTitle>Relationship Visualization</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="flex items-center justify-center h-[600px]">
              <div className="text-center">
                <RefreshCw className="w-8 h-8 mx-auto mb-4 text-gray-400 animate-spin" />
                <p className="text-gray-500">Loading graph data...</p>
              </div>
            </div>
          ) : (
            <CytoscapeGraph
              data={graphData}
              onNodeClick={handleNodeClick}
              onEdgeClick={handleEdgeClick}
              selectedNodeId={selectedNodeId}
              layout={layout}
            />
          )}
        </CardContent>
      </Card>

      {/* Legend */}
      <Card>
        <CardHeader>
          <CardTitle>Legend</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-blue-500" />
              <span className="text-sm">Server</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-green-500" />
              <span className="text-sm">Database</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-purple-500" />
              <span className="text-sm">Application</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-amber-500" />
              <span className="text-sm">Network</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-pink-500" />
              <span className="text-sm">Storage</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-cyan-500" />
              <span className="text-sm">Container</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-indigo-500" />
              <span className="text-sm">Service</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 rounded-full bg-gray-500" />
              <span className="text-sm">Other</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
