import MainLayout from "@layouts/main-layout";

export default function NotFound() {
  return (
    <MainLayout>
      <h1>Your Lost, Route Not Found</h1>
      <a href="/">Back home</a>
    </MainLayout>
  );
}
