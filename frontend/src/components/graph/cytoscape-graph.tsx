'use client';

import React, { useEffect, useRef, useState } from 'react';
import cytoscape, { Core, NodeSingular } from 'cytoscape';

export interface GraphNode {
  id: string;
  name: string;
  ci_type: string;
  ci_type_id: string;
  attributes: Record<string, any>;
}

export interface GraphEdge {
  id?: string;
  relationship_type: string;
  from_node_id: string;
  to_node_id: string;
  from_ci_type: string;
  to_ci_type: string;
  attributes: Record<string, any>;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

interface CytoscapeGraphProps {
  data: GraphData;
  onNodeClick?: (node: GraphNode) => void;
  onEdgeClick?: (edge: GraphEdge) => void;
  selectedNodeId?: string | null;
  layout?: 'cose' | 'circle' | 'grid' | 'breadthfirst' | 'concentric';
}

// Color palette for different CI types
const CI_TYPE_COLORS: Record<string, string> = {
  Server: '#3B82F6',      // Blue
  Database: '#10B981',    // Green
  Application: '#8B5CF6', // Purple
  Network: '#F59E0B',     // Amber
  Storage: '#EC4899',     // Pink
  Container: '#06B6D4',   // Cyan
  Service: '#6366F1',     // Indigo
  default: '#6B7280',     // Gray
};

export const CytoscapeGraph: React.FC<CytoscapeGraphProps> = ({
  data,
  onNodeClick,
  onEdgeClick,
  selectedNodeId,
  layout = 'cose',
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const cyRef = useRef<Core | null>(null);
  const [isReady, setIsReady] = useState(false);

  // Initialize Cytoscape
  useEffect(() => {
    if (!containerRef.current) return;

    // Create Cytoscape instance
    const cy = cytoscape({
      container: containerRef.current,
      style: [
        {
          selector: 'node',
          style: {
            'background-color': (ele: NodeSingular) => {
              const ciType = ele.data('ci_type') as string;
              return CI_TYPE_COLORS[ciType] || CI_TYPE_COLORS.default;
            },
            'label': 'data(name)',
            'width': 40,
            'height': 40,
            'font-size': '12px',
            'text-valign': 'bottom',
            'text-halign': 'center',
            'text-margin-y': 5,
            'color': '#374151',
            'text-outline-width': 2,
            'text-outline-color': '#ffffff',
            'border-width': 2,
            'border-color': '#ffffff',
            'overlay-padding': 6,
          },
        },
        {
          selector: 'node:selected',
          style: {
            'border-width': 4,
            'border-color': '#EF4444',
            'background-color': '#DC2626',
          },
        },
        {
          selector: 'node.highlighted',
          style: {
            'border-width': 4,
            'border-color': '#F59E0B',
            'background-color': '#F59E0B',
          },
        },
        {
          selector: 'edge',
          style: {
            'width': 2,
            'line-color': '#9CA3AF',
            'target-arrow-color': '#9CA3AF',
            'target-arrow-shape': 'triangle',
            'curve-style': 'bezier',
            'label': 'data(relationship_type)',
            'font-size': '10px',
            'text-rotation': 'autorotate',
            'text-margin-y': -10,
            'color': '#6B7280',
            'text-outline-width': 2,
            'text-outline-color': '#ffffff',
          },
        },
        {
          selector: 'edge:selected',
          style: {
            'line-color': '#EF4444',
            'target-arrow-color': '#EF4444',
            'width': 3,
          },
        },
      ],
      minZoom: 0.2,
      maxZoom: 3,
      wheelSensitivity: 0.2,
    });

    cyRef.current = cy;

    // Handle node clicks
    cy.on('tap', 'node', (event) => {
      const node = event.target;
      const nodeData: GraphNode = {
        id: node.data('id'),
        name: node.data('name'),
        ci_type: node.data('ci_type'),
        ci_type_id: node.data('ci_type_id'),
        attributes: node.data('attributes'),
      };
      onNodeClick?.(nodeData);
    });

    // Handle edge clicks
    cy.on('tap', 'edge', (event) => {
      const edge = event.target;
      const edgeData: GraphEdge = {
        relationship_type: edge.data('relationship_type'),
        from_node_id: edge.data('source'),
        to_node_id: edge.data('target'),
        from_ci_type: edge.data('from_ci_type'),
        to_ci_type: edge.data('to_ci_type'),
        attributes: edge.data('attributes'),
      };
      onEdgeClick?.(edgeData);
    });

    setIsReady(true);

    return () => {
      cy.destroy();
    };
  }, [onNodeClick, onEdgeClick]);

  // Update graph data when it changes
  useEffect(() => {
    if (!cyRef.current || !isReady) return;

    const cy = cyRef.current;

    // Clear existing elements
    cy.elements().remove();

    // Add nodes
    const nodes = data.nodes.map((node) => ({
      data: {
        id: node.id,
        name: node.name,
        ci_type: node.ci_type,
        ci_type_id: node.ci_type_id,
        attributes: node.attributes,
      },
    }));

    // Add edges
    const edges = data.edges.map((edge, index) => ({
      data: {
        id: edge.id || `edge-${index}`,
        source: edge.from_node_id,
        target: edge.to_node_id,
        relationship_type: edge.relationship_type,
        from_ci_type: edge.from_ci_type,
        to_ci_type: edge.to_ci_type,
        attributes: edge.attributes,
      },
    }));

    cy.add([...nodes, ...edges]);

    // Apply layout
    cy.layout({
      name: layout,
      animate: true,
      animationDuration: 500,
      // COSE layout options
      nodeRepulsion: 400000,
      idealEdgeLength: 100,
      edgeElasticity: 100,
      nestingFactor: 1.2,
      gravity: 1,
      numIter: 1000,
      // Other layouts
      fit: true,
      padding: 50,
    }).run();
  }, [data, layout, isReady]);

  // Highlight selected node
  useEffect(() => {
    if (!cyRef.current || !isReady) return;

    const cy = cyRef.current;

    // Remove all highlights
    cy.nodes().removeClass('highlighted');

    if (selectedNodeId) {
      const node = cy.getElementById(selectedNodeId);
      if (node.length > 0) {
        node.addClass('highlighted');
        // Center on the node
        cy.animate({
          center: { eles: node },
          zoom: 1.5,
        }, {
          duration: 500,
        });
      }
    }
  }, [selectedNodeId, isReady]);

  // Expose methods for parent components
  useEffect(() => {
    if (!cyRef.current || !isReady) return;

    // Store cy instance on window for external access (optional)
    (window as any).cy = cyRef.current;
  }, [isReady]);

  return (
    <div className="relative w-full h-full">
      <div
        ref={containerRef}
        className="w-full h-full bg-gray-50 rounded-lg border border-gray-200"
        style={{ minHeight: '600px' }}
      />
      {data.nodes.length === 0 && (
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="text-center">
            <p className="text-gray-500 text-lg mb-2">No data to visualize</p>
            <p className="text-gray-400 text-sm">
              Create some CI assets and relationships to see them here
            </p>
          </div>
        </div>
      )}
    </div>
  );
};

export default CytoscapeGraph;
