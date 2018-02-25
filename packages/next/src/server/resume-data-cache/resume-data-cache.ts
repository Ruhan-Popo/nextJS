import {
  type UseCacheCacheStore,
  type FetchCacheStore,
  stringifyFetchCacheStore,
  stringifyUseCacheCacheStore,
  parseUseCacheCacheStore,
  parseFetchCacheStore,
} from './cache-store'

/**
 * An immutable version of the resume data cache used during rendering.
 * This cache is read-only and cannot be modified once created.
 */
export interface RenderResumeDataCache {
  /**
   * A read-only Map store for values cached by the 'use cache' React hook.
   * The 'set' operation is omitted to enforce immutability.
   */
  readonly cache: Omit<UseCacheCacheStore, 'set'>

  /**
   * A read-only Map store for cached fetch responses.
   * The 'set' operation is omitted to enforce immutability.
   */
  readonly fetch: Omit<FetchCacheStore, 'set'>
}

/**
 * A mutable version of the resume data cache used during pre-rendering.
 * This cache allows both reading and writing of cached values.
 */
export interface PrerenderResumeDataCache {
  /**
   * A mutable Map store for values cached by the 'use cache' React hook.
   * Supports both get and set operations to build the cache during pre-rendering.
   */
  readonly cache: UseCacheCacheStore

  /**
   * A mutable Map store for cached fetch responses.
   * Supports both get and set operations to build the cache during pre-rendering.
   */
  readonly fetch: FetchCacheStore
}

type ResumeStoreSerialized = {
  store: {
    cache: {
      [key: string]: any
    }
    fetch: {
      [key: string]: any
    }
  }
}

/**
 * Serializes a resume data cache into a JSON string for storage or transmission.
 * Handles both 'use cache' values and fetch responses.
 *
 * @param resumeDataCache - The immutable cache to serialize
 * @returns A Promise that resolves to the serialized cache as a JSON string, or 'null' if empty
 */
export async function stringifyResumeDataCache(
  resumeDataCache: RenderResumeDataCache | PrerenderResumeDataCache
): Promise<string> {
  if (resumeDataCache.fetch.size === 0 && resumeDataCache.cache.size === 0) {
    return 'null'
  }

  const json: ResumeStoreSerialized = {
    store: {
      fetch: Object.fromEntries(
        stringifyFetchCacheStore(resumeDataCache.fetch.entries())
      ),
      cache: Object.fromEntries(
        await stringifyUseCacheCacheStore(resumeDataCache.cache.entries())
      ),
    },
  }

  return JSON.stringify(json)
}

/**
 * Creates a new empty mutable resume data cache for pre-rendering.
 * Initializes fresh Map instances for both the 'use cache' and fetch caches.
 * Used at the start of pre-rendering to begin collecting cached values.
 *
 * @returns A new empty PrerenderResumeDataCache instance
 */
export function createPrerenderResumeDataCache(): PrerenderResumeDataCache {
  return {
    cache: new Map(),
    fetch: new Map(),
  }
}

/**
 * Creates an immutable render resume data cache from either:
 * 1. An existing prerender cache instance
 * 2. A serialized cache string
 *
 * @param prerenderResumeDataCache - A PrerenderResumeDataCache instance to convert to immutable
 * @param persistedCache - A serialized cache string to parse
 * @returns An immutable RenderResumeDataCache instance
 */
export function createRenderResumeDataCache(
  prerenderResumeDataCache: PrerenderResumeDataCache
): RenderResumeDataCache
export function createRenderResumeDataCache(
  persistedCache: string
): RenderResumeDataCache
export function createRenderResumeDataCache(
  prerenderResumeDataCacheOrPersistedCache: PrerenderResumeDataCache | string
): RenderResumeDataCache {
  if (typeof prerenderResumeDataCacheOrPersistedCache !== 'string') {
    // If the cache is already a prerender cache, we can return it directly,
    // we're just performing a type change.
    return prerenderResumeDataCacheOrPersistedCache
  }

  if (prerenderResumeDataCacheOrPersistedCache === 'null') {
    return {
      cache: new Map(),
      fetch: new Map(),
    }
  }

  const json: ResumeStoreSerialized = JSON.parse(
    prerenderResumeDataCacheOrPersistedCache
  )
  return {
    cache: parseUseCacheCacheStore(Object.entries(json.store.cache)),
    fetch: parseFetchCacheStore(Object.entries(json.store.fetch)),
  }
}
