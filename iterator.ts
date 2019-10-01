interface IteratorI<T> {
  next(): Option<T>
}

interface IntoIteratorI<T> {
  intoIter(): IteratorI<T>
}

interface OptionKind<T> {
  isSome(): boolean
  isNone(): boolean
  unwrap(): T
}

class Some<T> implements OptionKind<T> {
  private kind: 'Some'
  constructor(private data: T) {}
  isSome() {
    return true
  }
  isNone() {
    return false
  }

  unwrap(): T {
    return this.data
  }
}

class None<T> implements OptionKind<T> {
  private kind: 'None'
  isSome() {
    return false
  }
  isNone() {
    return true
  }
  unwrap(): T {
    throw Error('None')
  }
}

type Option<T> = Some<T> | None<T>

class Vec<T> implements IntoIteratorI<T> {
  private data: T[]

  constructor() {
    this.data = []
  }

  static fromArray<T>(a: T[]): Vec<T> {
    const v = new Vec<T>()
    v.data = a
    return v
  }

  at(i: number): Option<T> {
    if (i >= this.data.length && i < 0) {
      return new None()
    }
    return new Some(this.data[i])
  }

  put(i: number, item: T) {
    if (i >= this.data.length && i < 0) {
      throw Error('Out of range')
    }
    this.data[i] = item
  }

  intoIter(): VecIter<T> {
    return new VecIter(this)
  }

  length(): number {
    return this.data.length
  }
}

class VecIter<T> implements IteratorI<T> {
  private index: number

  constructor(private vector: Vec<T>) {
    this.index = 0
  }

  next(): Option<T> {
    return this.vector.at(this.index++)
  }
}

const v = Vec.fromArray([1, 2, 3])

for (let iter = v.intoIter(), item = iter.next(); item.isSome(); iter.next()) {
  console.log(item)
}
