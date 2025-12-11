'use client';

import React from 'react';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Filter } from 'lucide-react';

interface CIType {
  id: string;
  name: string;
}

interface CITypeFilterProps {
  ciTypes: CIType[];
  selectedType: string | null;
  onTypeChange: (typeId: string | null) => void;
  isLoading?: boolean;
}

export const CITypeFilter: React.FC<CITypeFilterProps> = ({
  ciTypes,
  selectedType,
  onTypeChange,
  isLoading = false,
}) => {
  const handleValueChange = (value: string) => {
    if (value === 'all') {
      onTypeChange(null);
    } else {
      onTypeChange(value);
    }
  };

  return (
    <div className="flex items-center gap-2">
      <Filter className="w-4 h-4 text-gray-500" />
      <Select
        value={selectedType || 'all'}
        onValueChange={handleValueChange}
        disabled={isLoading}
      >
        <SelectTrigger className="w-[200px]">
          <SelectValue placeholder="Filter by CI Type" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All CI Types</SelectItem>
          {ciTypes.map((type) => (
            <SelectItem key={type.id} value={type.name}>
              {type.name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
};

export default CITypeFilter;
