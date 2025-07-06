// work in progress datastore module for intergration with a local cache and redis
//
// layers:
// local cache (instant quick access for hot data with a TTL)
// redis (a little less quick since over TCP, but is a little hotter and longer lived)
// postgres (long term data storage)
//
// cache invalidation:
// make a central Datastore struct for all caches and be able to do .restore(CACHEID) and the cache entries are modified by pulling PGSQL data.

pub mod cache;
pub mod id;
pub mod ttl;
