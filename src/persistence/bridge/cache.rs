use redis::Commands;
use crate::persistence::bridge::{Bridge, CachedRecord, CACHE_TTL_SEC};

impl Bridge {
    pub fn cache_set_obj(&mut self, record: &CachedRecord) {
        let key = Self::key_obj(record.id);
        if let Ok(json) = serde_json::to_string(record) {
            let _: Result<(), _> = self.redis.set_ex(&key, &json, CACHE_TTL_SEC);
        }
    }

    pub fn cache_link_from(&mut self, from_id: i64, store_id: i64) {
        let from_key = Self::key_from(from_id);
        let rev_key = Self::key_rev(store_id);
        let _: Result<(), _> = self.redis.sadd(&from_key, store_id);
        let _: Result<(), _> = self.redis.expire(&from_key, CACHE_TTL_SEC as i64);
        let _: Result<(), _> = self.redis.sadd(&rev_key, &from_key);
        let _: Result<(), _> = self.redis.expire(&rev_key, CACHE_TTL_SEC as i64);
    }

    pub fn cache_link_search(&mut self, search_key: &str, store_id: i64) {
        let rev_key = Self::key_rev(store_id);
        let _: Result<(), _> = self.redis.sadd(search_key, store_id);
        let _: Result<(), _> = self.redis.expire(search_key, CACHE_TTL_SEC as i64);
        let _: Result<(), _> = self.redis.sadd(&rev_key, search_key);
        let _: Result<(), _> = self.redis.expire(&rev_key, CACHE_TTL_SEC as i64);
    }

    pub fn cache_get_obj(&mut self, id: i64) -> Option<CachedRecord> {
        let key = Self::key_obj(id);
        let json: Option<String> = self.redis.get(&key).ok()?;
        let json = json?;
        let _: Result<(), _> = self.redis.expire(&key, CACHE_TTL_SEC as i64);
        serde_json::from_str(&json).ok()
    }

    pub fn invalidate_by_id(&mut self, id: i64) {
        let obj_key = Self::key_obj(id);
        let rev_key = Self::key_rev(id);

        let ref_keys: Vec<String> = self.redis.smembers(&rev_key).unwrap_or_default();
        for rk in &ref_keys {
            let _: Result<(), _> = self.redis.srem(rk, id);
        }

        let _: Result<(), _> = self.redis.del(&obj_key);
        let _: Result<(), _> = self.redis.del(&rev_key);
    }
}