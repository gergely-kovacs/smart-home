import { graphql } from "@/gql/gql";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import request from "graphql-request";

export const Route = createFileRoute("/")({
  component: App,
});

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

const SITE_ID = 1;

function App() {
  const { data } = useQuery({
    queryKey: ["site", SITE_ID],
    queryFn: async () => {
      const result = await request(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        GET_SITE_WITH_ALL_MODELS,
        {
          siteId: SITE_ID,
        },
      );
      return result.site;
    },
  });

  return (
    <main className="grow flex flex-col items-center justify-center bg-[#282c34]"></main>
  );
}
