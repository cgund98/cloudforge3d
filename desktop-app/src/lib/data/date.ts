// Parse a date string returned by a protobuf
export const parseProtoDate = (protoDate: string) =>
  new Date(protoDate.slice(0, 23) + "Z");

export const formatDuration = (duration: number) => {
  const seconds = Math.max(0, Math.floor((duration / 1000) % 60));
  const minutes = Math.max(0, Math.floor((duration / (1000 * 60)) % 60));
  const hours = Math.max(0, Math.floor((duration / (1000 * 60 * 60)) % 24));

  const paddedHours = hours < 10 ? "0" + hours : hours;
  const paddedMinutes = minutes < 10 ? "0" + minutes : minutes;
  const paddedSeconds = seconds < 10 ? "0" + seconds : seconds;
  return `${paddedHours}:${paddedMinutes}:${paddedSeconds}`;
};
