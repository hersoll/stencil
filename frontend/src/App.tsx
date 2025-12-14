import {
  createBrowserRouter,
  RouterProvider,
} from "react-router";
import './App.css';
import HomePage from "./pages/HomePage";
import CoursePage, { courseLoader } from "./pages/CoursePage";

let router = createBrowserRouter([
  {
    path: "/",
    Component: HomePage,
  },
  {
    path: "/:courseId",
    Component: CoursePage,
    loader: courseLoader
  }
]);

function App() {

  return <RouterProvider router={router} />;
}

export default App
