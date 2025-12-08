-- Valuation Records table
CREATE TABLE valuation_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ci_asset_id UUID NOT NULL REFERENCES ci_assets(id),
    initial_value DECIMAL(15,2) NOT NULL,
    current_value DECIMAL(15,2) NOT NULL,
    useful_life_years INTEGER NOT NULL,
    depreciation_method VARCHAR(50) NOT NULL DEFAULT 'straight_line',
    purchase_date DATE,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(ci_asset_id)
);

-- Amortization Entries table
CREATE TABLE amortization_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    valuation_id UUID NOT NULL REFERENCES valuation_records(id),
    year INTEGER NOT NULL,
    opening_value DECIMAL(15,2) NOT NULL,
    depreciation_amount DECIMAL(15,2) NOT NULL,
    closing_value DECIMAL(15,2) NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(valuation_id, year)
);

-- Indexes for valuation records
CREATE INDEX idx_valuation_asset_id ON valuation_records(ci_asset_id);
CREATE INDEX idx_valuation_useful_life ON valuation_records(useful_life_years);

-- Indexes for amortization entries
CREATE INDEX idx_amortization_valuation_id ON amortization_entries(valuation_id);
CREATE INDEX idx_amortization_year ON amortization_entries(year);
CREATE INDEX idx_amortization_closing_value ON amortization_entries(closing_value);

-- Triggers for updated_at
CREATE TRIGGER update_valuation_records_updated_at BEFORE UPDATE ON valuation_records
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();