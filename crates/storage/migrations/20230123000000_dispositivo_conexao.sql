-- Ligação configurável por dispositivo (spec 05-integrations/01-hardware).
-- `conexao_tipo`: file | null | tcp | serial | windows_spooler
-- `conexao_config`: JSON com os parâmetros (host/port, porta COM/baud, nome da
-- fila Windows, etc.). Interpretado por `devices::transport::Connection`.
ALTER TABLE dispositivos ADD COLUMN conexao_tipo TEXT NOT NULL DEFAULT 'file';
ALTER TABLE dispositivos ADD COLUMN conexao_config TEXT NOT NULL DEFAULT '{}';

-- Dispositivos existentes file-based migram para uma ligação `file`.
UPDATE dispositivos
   SET conexao_config = json_object('path', output_path)
 WHERE output_path IS NOT NULL AND output_path <> '';
