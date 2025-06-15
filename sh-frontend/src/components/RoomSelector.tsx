import { Room } from "@/gql/graphql";
import { createListCollection, Select, Stack } from "@chakra-ui/react";

export default function RoomSelector({
  rooms,
  onChange,
}: {
  rooms: Room[];
  onChange: (roomId: number) => void;
}) {
  const roomCollection = createListCollection({
    items: rooms,
    itemToString: (room) => room.name,
    itemToValue: (room) => room.id.toString(),
  });

  return (
    <Stack gap="{4}" width="{320px}">
      <Select.Root
        variant="subtle"
        collection={roomCollection}
        onValueChange={(e) => onChange(parseInt(e.value[0]))}
      >
        <Select.HiddenSelect />

        <Select.Label>Room</Select.Label>

        <Select.Control>
          <Select.Trigger>
            <Select.ValueText placeholder="Select a room" />
          </Select.Trigger>
        </Select.Control>

        <Select.Positioner>
          <Select.Content>
            {roomCollection.items.map((room) => (
              <Select.Item item={room} key={room.id}>
                {room.name}
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Positioner>
      </Select.Root>
    </Stack>
  );
}
