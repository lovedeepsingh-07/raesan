import { z } from "zod";

export const QuestionOptionModel = z.object({
	id: z.uuidv4(),
	question_id: z.string(),
	key: z.string(),
	value: z.string()
});
export type QuestionOption = z.infer<typeof QuestionOptionModel>;

export const QuestionTypeModel = z.enum(["MCQ", "INTEGER"]);
export type QuestionType = z.infer<typeof QuestionTypeModel>;

export const QuestionModel = z.object({
	id: z.uuidv4(),
	exam_key: z.string(),
	subject_key: z.string(),
	chapter_id: z.string(),
	chapter_key: z.string(),
	chapter_group: z.string(),
	question_type: QuestionTypeModel,
	content: z.string(),
	options: z.array(QuestionOptionModel).default([]),
	answer: z.string()
});
export type Question = z.infer<typeof QuestionModel>;

export const ChapterModel = z.object({
	id: z.uuidv4(),
	key: z.string(),
	exam_key: z.string(),
	subject_id: z.string(),
	subject_key: z.string(),
	title: z.string(),
	group: z.string(),
	questions: z.array(QuestionModel).default([])
});
export type Chapter = z.infer<typeof ChapterModel>;

export const SubjectModel = z.object({
	id: z.uuidv4(),
	key: z.string(),
	exam_id: z.string(),
	exam_key: z.string(),
	title: z.string(),
	chapters: z.array(ChapterModel).default([])
});
export type Subject = z.infer<typeof SubjectModel>;

export const ExamModel = z.object({
	id: z.uuidv4(),
	exam: z.string(),
	title: z.string(),
	group: z.string(),
	subjects: z.array(SubjectModel).default([])
});
export type Exam = z.infer<typeof ExamModel>;
