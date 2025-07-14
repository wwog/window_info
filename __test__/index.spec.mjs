import test from 'ava'

import { listWindows } from '../index.js'

test('listWindows from native', (t) => {
  const windows = listWindows()
  t.true(Array.isArray(windows), 'listWindows should return an array')
  t.true(windows.length > 0, 'listWindows should return at least one window')
  windows.forEach((window) => {
    t.true(typeof window === 'string', 'Each window should be a string')
  })
})
