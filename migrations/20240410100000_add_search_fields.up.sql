ALTER TABLE properties
ADD COLUMN address VARCHAR(255),
ADD COLUMN city VARCHAR(100),
ADD COLUMN state VARCHAR(100),
ADD COLUMN zip_code VARCHAR(20),
ADD COLUMN country VARCHAR(100),
ADD COLUMN latitude DECIMAL(9,6),
ADD COLUMN longitude DECIMAL(9,6),
ADD COLUMN price DECIMAL(15,2),
ADD COLUMN availability VARCHAR(20) DEFAULT 'Available' CHECK (availability IN ('Available', 'Unavailable', 'Reserved'));
