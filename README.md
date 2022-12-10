# cheeper-rs
Implementation of some messaging service using actix-web & OpenSearch serving data.

NOTE: I know that using search engine as database was bad idea.

# Disclaimer
Most likely, it's not the best Rust over there. 

Just testing thing, nothing serious.

# CLI Args
| Arg | Description                |
|-----|----------------------------|
| -e  | OpenSearch URL             |
| -u  | OpenSearch username        |
| -p  | OpenSearch user's password |

-u & -p are needed even if OpenSearch is running in insecure mode.

In that case, just pass random strings.