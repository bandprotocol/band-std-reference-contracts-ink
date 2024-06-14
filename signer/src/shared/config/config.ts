import { registerAs } from '@nestjs/config'
import { readFileSync } from 'fs'
import { camelizeKeys } from 'humps'
import { join } from 'path'

const CONFIG_FILENAME = 'config.json'

export default registerAs('chain', () => {
  return camelizeKeys(JSON.parse(readFileSync(join(__dirname, CONFIG_FILENAME), 'utf8'))) as Record<string, any>
})
