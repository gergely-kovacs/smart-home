import DeviceSelector from "@/components/DeviceSelector";
import RoomSelector from "@/components/RoomSelector";
import SensorReadingTable from "@/components/SensorReadingTable";
import SiteSelector from "@/components/SiteSelector";
import { graphql } from "@/gql/gql";
import { Device, SensorReading } from "@/gql/graphql";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import request from "graphql-request";
import { useEffect, useState } from "react";

export const Route = createFileRoute("/")({
  component: App,
});

const GET_SITES = graphql(`
  query GetSites {
    sites {
      id
      name
    }
  }
`);

const GET_SITE_WITH_ALL_MODELS = graphql(`
  query GetSiteWithAllModels($siteId: Int!) {
    site(id: $siteId) {
      id
      name
      address
      rooms {
        id
        name
        devices {
          id
          name
          deviceType
          uniqueIdentifier
          sensorReadings {
            id
            value
            unit
            timestamp
          }
          controlSetpoints {
            id
            setpointType
            value
            unit
            timestamp
          }
        }
      }
    }
  }
`);

function App() {
  const [selectedSiteId, setSelectedSiteId] = useState<number | null>(null);
  const [selectedRoomId, setSelectedRoomId] = useState<number | null>(null);
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedDeviceId, setSelectedDeviceId] = useState<number | null>(null);
  const [readings, setReadings] = useState<SensorReading[]>([]);

  const {
    status: sitesStatus,
    data: sites,
    error: sitesError,
  } = useQuery({
    queryKey: ["sites"],
    queryFn: async () => {
      const result = await request(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        GET_SITES,
      );
      return result.sites;
    },
  });

  const {
    status: siteStatus,
    data: site,
    error: siteError,
  } = useQuery({
    queryKey: ["site", selectedSiteId],
    queryFn: async () => {
      const result = await request(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        GET_SITE_WITH_ALL_MODELS,
        {
          siteId: selectedSiteId as number,
        },
      );
      return result.site;
    },
    enabled: selectedSiteId !== null,
  });

  function handleSiteChange(siteId: number) {
    setSelectedSiteId(siteId);
  }

  function handleRoomChange(roomId: number) {
    setSelectedRoomId(roomId);
  }

  function handleDeviceChange(deviceId: number) {
    setSelectedDeviceId(deviceId);
  }

  useEffect(() => {
    const room = site?.rooms.find((room) => room.id === selectedRoomId);
    setDevices(room?.devices || []);
  }, [selectedRoomId]);

  useEffect(() => {
    const device = devices.find((device) => device.id === selectedDeviceId);
    setReadings(device?.sensorReadings || []);
  }, [selectedDeviceId]);

  if (sitesStatus === "pending") {
    return <div>Loading...</div>;
  }

  if (sitesStatus === "error") {
    return <div>Error: {sitesError?.message}</div>;
  }

  return (
    <main className="grow flex flex-col gap-4 bg-[#282c34] p-4">
      {!sites ? (
        <div>No sites have been created yet</div>
      ) : (
        <>
          <div className="flex flex-wrap gap-4">
            <SiteSelector sites={sites} onChange={handleSiteChange} />

            {site && (
              <RoomSelector rooms={site.rooms} onChange={handleRoomChange} />
            )}

            {site && selectedRoomId && (
              <DeviceSelector devices={devices} onChange={handleDeviceChange} />
            )}
          </div>

          {selectedDeviceId && <SensorReadingTable readings={readings} />}
        </>
      )}
    </main>
  );
}
