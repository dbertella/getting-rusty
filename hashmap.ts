const BUCKET_SIZE = 4

const hashing = <K>(str: K) => 10000

class HashMapIter<K, T> implements IteratorI<[K, T]> {
  private iter: VecIter<Option<[K, T]>>
  next(): Option<[K, T]> {
    while (true) {
      const outer = this.iter.next()
      if (outer.isNone()) {
        return new None()
      }
      const inner = outer.unwrap()
      if (inner.isSome()) {
        return inner
      }
    }
  }
}

class HashMap<K, T> {
  private data: Vec<Option<[K, T]>>

  constructor() {
    this.data = Vec.fromArray(Array.prototype.fill(None, 0, 16 * BUCKET_SIZE))
  }

  calcIndex(key: K): number {
    return (hashing(key) % this.data.length()) * BUCKET_SIZE
  }

  insert(item: [K, T]) {
    const [key] = item
    const index = this.calcIndex(key)
    for (let i = index; i < index + BUCKET_SIZE; ++i) {
      const el = this.data.at(i).unwrap()
      if (el.isNone()) {
        this.data.put(i, new Some(item))
        return
      } else {
        const [kk] = el.unwrap()
        if (kk === key) {
          this.data.put(i, new Some(item))
          return
        }
      }
    }
    throw Error('No Space')
  }

  at(key: K): Option<T> {
    const index = this.calcIndex(key)
    for (let i = index; i < index + BUCKET_SIZE; ++i) {
      const item = this.data.at(i).unwrap()
      if (item.isSome()) {
        const [kk, value] = item.unwrap()
        if (kk === key) {
          return new Some(value)
        }
      } else {
        break
      }
    }
    return new None()
  }
}
