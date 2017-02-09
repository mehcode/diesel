initSidebarItems({"struct":[["Array","The Array SQL type. This wraps another type to represent a SQL array of that type. Multidimensional arrays are not supported, nor are arrays containing null."],["Oid","The OID SQL type. This is a PostgreSQL specific type."],["Timestamptz","The \"timestamp with time zone\" SQL type, which PostgreSQL abbreviates to `timestamptz`."],["Uuid","The UUID SQL type. This type can only be used with `feature = \"uuid\"`"]],"type":[["BigSerial","Alias for BigInt"],["Bytea","Alias for `Binary`, to ensure `infer_schema!` works"],["Serial","Alias for Integer"],["SmallSerial","Alias for SmallInt"]]});