import { Device } from "@/gql/graphql";
import { createListCollection, Select, Stack } from "@chakra-ui/react";

export default function DeviceSelector({
  devices,
  onChange,
}: {
  devices: Device[];
  onChange: (deviceId: number) => void;
}) {
  const deviceCollection = createListCollection({
    items: devices,
    itemToString: (device) => device.name,
    itemToValue: (device) => device.id.toString(),
  });

  return (
    <Stack gap="{4}" width="{320px}">
      <Select.Root
        variant="subtle"
        collection={deviceCollection}
        onValueChange={(e) => onChange(parseInt(e.value[0]))}
      >
        <Select.HiddenSelect />

        <Select.Label>Device</Select.Label>

        <Select.Control>
          <Select.Trigger>
            <Select.ValueText placeholder="Select a device" />
          </Select.Trigger>
        </Select.Control>

        <Select.Positioner>
          <Select.Content>
            {deviceCollection.items.map((device) => (
              <Select.Item item={device} key={device.id}>
                {device.name}
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Positioner>
      </Select.Root>
    </Stack>
  );
}
