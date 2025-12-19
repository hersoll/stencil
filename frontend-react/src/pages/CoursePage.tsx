import { useLoaderData, useParams, type LoaderFunctionArgs } from "react-router-dom";
import PDFButton from "../components/PDFButton";
import i18n from '../i18n.ts';

interface CourseData {
  id: string;
  name: string;
  desc: string;
  chapters: [ChapterData];
}

interface ChapterData {
  id: string;
  name: string;
  desc: string;
  topics: [TopicData];
}

interface TopicData {
  id: string;
  name: string;
  desc: string;
}

export default function CoursePage() {
  const course = useLoaderData() as CourseData;
  const { courseId } = useParams();

  return (
    <div>
      <h1 className="mb-4">Course {courseId}</h1>
      <p>{course.desc}</p>
      <PDFButton />
    </div>
  )
}

export async function courseLoader(courseId: string) {
  const API_URL = import.meta.env.VITE_API_URL || '/api';
  if (!courseId) {
    throw new Response("Missing course ID", { status: 400 });
  }
  const res = await fetch(`${API_URL}/${i18n.language}/course/${courseId}`);

  if (!res.ok) {
    throw new Response(res.statusText || "Not found", { status: 404 });
  }

  return res.json();
}
