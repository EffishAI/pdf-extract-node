import test from 'ava'

import { extractText } from '../index.js'

test('extractText from native', (t) => {
  t.is(extractText(Buffer.from("")), "")
})
