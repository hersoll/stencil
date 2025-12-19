import { useNavigate } from "react-router-dom";

interface ButtonProps {
  // The name on the button
  course: string;
  page: string;
}
export default function CourseButton({ course, page }: ButtonProps) {
  const navigate = useNavigate();

  return (
    <>
      <button onClick={() => navigate("/" + page)}>
        {course}
      </button >
    </>
  )
}
