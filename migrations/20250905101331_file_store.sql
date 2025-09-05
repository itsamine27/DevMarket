
-- Add a BYTEA column to store executable binary data
ALTER TABLE Product
ADD COLUMN executable BYTEA;

-- Add a CHECK constraint to limit the size of the binary data to 5 MB
ALTER TABLE Product
ADD CONSTRAINT executable_size_check
CHECK (octet_length(executable) <= 5 * 1024 * 1024);
