import { SensorReading, SensorUnit } from "@/gql/graphql";
import {
  ButtonGroup,
  IconButton,
  Pagination,
  Stack,
  Table,
} from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { LuChevronLeft, LuChevronRight } from "react-icons/lu";

function get_label_for_unit(unit: SensorUnit | null | undefined): string {
  switch (unit) {
    case SensorUnit.Celsius:
      return "(°C)";
    case SensorUnit.Fahrenheit:
      return "(°F)";
    default:
      return "";
  }
}

export default function SensorReadingTable({
  readings,
}: {
  readings: SensorReading[];
}) {
  if (!readings) {
    return <div>No readings have been recorded yet</div>;
  }

  const unit = get_label_for_unit(readings[0]?.unit);
  const pageSize = 10;

  const [page, setPage] = useState(1);
  const [currentPageReadings, setCurrentPageReadings] = useState<Reading[]>([]);

  useEffect(() => {
    const start = (page - 1) * pageSize;
    const end = start + pageSize;
    setCurrentPageReadings(readings.slice(start, end));
  }, [page, readings]);

  return (
    <Stack className="w-full max-w-xl self-center" gap="5">
      <Table.ScrollArea borderWidth="1px" maxW="xl">
        <Table.Root variant="line" striped showColumnBorder>
          <Table.Header>
            <Table.Row>
              <Table.ColumnHeader>Timestamp</Table.ColumnHeader>
              <Table.ColumnHeader>Value {unit}</Table.ColumnHeader>
            </Table.Row>
          </Table.Header>

          <Table.Body>
            {currentPageReadings.map((reading) => (
              <Table.Row key={reading.id}>
                <Table.Cell>
                  {new Date(reading.timestamp).toLocaleString()}
                </Table.Cell>
                <Table.Cell>{reading.value}</Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Root>
      </Table.ScrollArea>

      <Pagination.Root
        className="self-center lg:self-end"
        count={readings.length}
        pageSize={pageSize}
        page={page}
        onPageChange={(e) => setPage(e.page)}
      >
        <ButtonGroup variant="subtle" size="sm" wrap="wrap">
          <Pagination.PrevTrigger asChild>
            <IconButton>
              <LuChevronLeft />
            </IconButton>
          </Pagination.PrevTrigger>

          <Pagination.Items
            render={(page) => (
              <IconButton variant={{ base: "outline", _selected: "solid" }}>
                {page.value}
              </IconButton>
            )}
          />

          <Pagination.NextTrigger asChild>
            <IconButton>
              <LuChevronRight />
            </IconButton>
          </Pagination.NextTrigger>
        </ButtonGroup>
      </Pagination.Root>
    </Stack>
  );
}
