import { Center, Loader } from '@mantine/core'

function Unrecognized() {
  return (
    <Center style={{ height: '100vh' }}>
      <Loader size="xl" variant="dots" />
    </Center>
  )
}

export default Unrecognized
