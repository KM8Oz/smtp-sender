import test from 'ava'

import { send } from '../index.js'

test('send from native', (t) => {
  if(send)  t.pass()
})
