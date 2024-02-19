# RSCAN DESIGN

We have to have several things.

1. Views/Components using `ratatui` that display data.
2. Scanners/Modules/Handlers that retrieve data.
3. A way to dispatch/trigger handlers/scanners/modules.
4. Automated scanner registration.
5. Automated component/view registration.
6. Unified data models, plugins need to be able to mark data like Host IP Address, Default Gateway and both read and write to it.

There will be two roles/things that could need implemented.

1. Plugins which interact with models.
2. Models which are markers/containers for Fields.
3. Fields which are the actual data.

Fields should be responsible for retrieving their own data.

Models should register their fields
