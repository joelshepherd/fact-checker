import { OpinionReply } from "../api.ts";
import { sessionContext } from "../context/session.tsx";
import { React } from "../deps.ts";

interface Props {
  opinion: OpinionReply;
  onVote: () => void;
}

export default function Opinion({
  opinion,
  onVote,
}: Props): React.ReactElement {
  const session = React.useContext(sessionContext);

  return (
    <p>
      {opinion.body}
      {session.authenticated && <button onClick={onVote}>Support</button>}
    </p>
  );
}
