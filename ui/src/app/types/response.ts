export default interface ApiResponse<T> {
  ok: boolean,
  body: T | string
};
