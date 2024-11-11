import { FileRef, nextTestSetup } from 'e2e-utils'
import path from 'path'
import { sandbox } from 'development-sandbox'
import { outdent } from 'outdent'

describe('ReactRefreshModule app', () => {
  const { next } = nextTestSetup({
    files: new FileRef(path.join(__dirname, 'fixtures', 'default-template')),
    skipStart: true,
  })

  it('should allow any variable names', async () => {
    const { session, cleanup } = await sandbox(next, new Map([]))
    await session.assertNoRedbox()

    const variables = [
      '_a',
      '_b',
      'currentExports',
      'prevExports',
      'isNoLongerABoundary',
    ]

    for await (const variable of variables) {
      await session.patch(
        'app/page.js',
        outdent`
          'use client'
          import { default as ${variable} } from 'next/link'
          console.log({ ${variable} })
          export default function Page() {
            return null
          }
        `
      )
      await session.assertNoRedbox()
      expect(next.cliOutput).not.toContain(
        `'${variable}' has already been declared`
      )
    }

    await cleanup()
  })
})