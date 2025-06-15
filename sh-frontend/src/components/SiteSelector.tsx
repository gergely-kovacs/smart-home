import { Site } from "@/gql/graphql";
import { createListCollection, Select, Stack } from "@chakra-ui/react";

export default function SiteSelector({
  sites,
  onChange,
}: {
  sites: Site[];
  onChange: (siteId: number) => void;
}) {
  const siteCollection = createListCollection({
    items: sites,
    itemToString: (site) => site.name,
    itemToValue: (site) => site.id.toString(),
  });

  return (
    <Stack gap="{4}" width="{320px}">
      <Select.Root
        variant="subtle"
        collection={siteCollection}
        onValueChange={(e) => onChange(parseInt(e.value[0]))}
      >
        <Select.HiddenSelect />

        <Select.Label>Site</Select.Label>

        <Select.Control>
          <Select.Trigger>
            <Select.ValueText placeholder="Select a site" />
          </Select.Trigger>
        </Select.Control>

        <Select.Positioner>
          <Select.Content>
            {siteCollection.items.map((site) => (
              <Select.Item item={site} key={site.id}>
                {site.name}
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Positioner>
      </Select.Root>
    </Stack>
  );
}
