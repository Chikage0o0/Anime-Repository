import { useStore } from '@/store'
import {
  Modal,
  Button,
  Group,
  TextInput,
  Select,
  Divider,
  NumberInput,
  SegmentedControl,
  Center,
  Box,
} from '@mantine/core'
import { UseFormReturnType } from '@mantine/form'
import { showNotification } from '@mantine/notifications'
import { IconCheck, IconDeviceTv, IconMovie, IconX } from '@tabler/icons-react'
import { invoke } from '@tauri-apps/api'
import { flowResult } from 'mobx'
import { Dispatch, SetStateAction } from 'react'
import { useTranslation } from 'react-i18next'

function EditVideo({
  opened,
  setOpened,
  form,
}: {
  opened: boolean
  setOpened: Dispatch<SetStateAction<boolean>>
  form: UseFormReturnType<any, any>
}) {
  const {} = useStore()

  const { t } = useTranslation()
  return (
    <Modal
      size="lg"
      opened={opened}
      onClose={() => setOpened(false)}
      title={t('unrecognized_videos.video_info')}>
      <SegmentedControl
        fullWidth
        mb="xs"
        data={[
          {
            value: 'movie',
            label: (
              <Center>
                <IconMovie size={16} />
                <Box ml={10}>Movie</Box>
              </Center>
            ),
          },
          {
            value: 'tvshow',
            label: (
              <Center>
                <IconDeviceTv size={16} />
                <Box ml={10}>Tvshow</Box>
              </Center>
            ),
          },
        ]}
        {...form.getInputProps('type')}
      />
      <Group position="center" grow>
        <Button variant="outline" color="red" onClick={() => form.reset()}>
          {t('UI.reset')}
        </Button>
        <Button variant="outline" color="blue">
          {t('UI.submit')}
        </Button>
      </Group>
    </Modal>
  )
}

export default EditVideo
