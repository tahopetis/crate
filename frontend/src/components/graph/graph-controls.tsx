'use client';

import React from 'react';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { ZoomIn, ZoomOut, Maximize2, LayoutGrid } from 'lucide-react';

interface GraphControlsProps {
  onZoomIn: () => void;
  onZoomOut: () => void;
  onFit: () => void;
  onReset: () => void;
  layout: string;
  onLayoutChange: (layout: string) => void;
  nodeCount: number;
  edgeCount: number;
}

export const GraphControls: React.FC<GraphControlsProps> = ({
  onZoomIn,
  onZoomOut,
  onFit,
  onReset,
  layout,
  onLayoutChange,
  nodeCount,
  edgeCount,
}) => {
  return (
    <div className="flex items-center justify-between gap-4 p-4 bg-white border border-gray-200 rounded-lg shadow-sm">
      {/* Left side - Stats */}
      <div className="flex items-center gap-4 text-sm text-gray-600">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-blue-500" />
          <span className="font-medium">{nodeCount}</span>
          <span>nodes</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded bg-gray-400" />
          <span className="font-medium">{edgeCount}</span>
          <span>edges</span>
        </div>
      </div>

      {/* Center - Layout Selector */}
      <div className="flex items-center gap-2">
        <LayoutGrid className="w-4 h-4 text-gray-500" />
        <Select value={layout} onValueChange={onLayoutChange}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Select layout" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="cose">Force-Directed (COSE)</SelectItem>
            <SelectItem value="circle">Circle</SelectItem>
            <SelectItem value="grid">Grid</SelectItem>
            <SelectItem value="breadthfirst">Breadth First</SelectItem>
            <SelectItem value="concentric">Concentric</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {/* Right side - Zoom Controls */}
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onClick={onZoomIn}
          title="Zoom In"
        >
          <ZoomIn className="w-4 h-4" />
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={onZoomOut}
          title="Zoom Out"
        >
          <ZoomOut className="w-4 h-4" />
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={onFit}
          title="Fit to Screen"
        >
          <Maximize2 className="w-4 h-4" />
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={onReset}
          title="Reset View"
        >
          Reset
        </Button>
      </div>
    </div>
  );
};

export default GraphControls;
