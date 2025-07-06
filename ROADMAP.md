### Roadmap for `birb`

- Build first, optimize later - Value building features right and well first, and then worry about optimizations later, it's better to have something working that spending hours upon hours doing optimizations.

### Roadmap:
- [ ] Make docker compose image
- [ ] Error handling refactoring - Make central fatality system where services can report they are performing auto maintenance, or that they have encoutered a fatal error, and has to request for their routes to be detached (this might require a custom router system with hyper or another low level web framework)
- [ ] Adding a caching system - work has started in birb/datastore and will use composed DashMap's with a cache invalidation architecture where cache.restore(CacheIdentifier) will grab new data from the database and update the data, still keeping the active TTL. (this will evetually replace BlogSchema)
- [ ] Add AuthService and finish AuthSchema system.
- [ ] Refactor the current Sql Table handle system for a better optimized solution which isn't handles which are one off used and dropped.
- [ ] Add frontend (svelte or rust based wasm SSR) (after backend)
- [ ] Add post tags
