import {
  createBrowserRouter,
  RouterProvider,
} from "react-router";
import './App.css';
import HomePage from "./pages/HomePage";
import CoursePage, { courseLoader } from "./pages/CoursePage";
import AdminPage from "./pages/AdminPage";

let router = createBrowserRouter([
  {
    path: "/",
    Component: HomePage,
  },
  {
    path: "/admin",
    Component: AdminPage,
  },
  // Layout and children for each course?
  {
    path: "/1b",
    handle: { courseId: "1b" },
    Component: CoursePage,
    loader: () => courseLoader("1b"),
  },
  {
    path: "/2b",
    handle: { courseId: "2b" },
    Component: CoursePage,
    loader: () => courseLoader("2b"),
  }

]);

function App() {

  return <RouterProvider router={router} />;
}

export default App
